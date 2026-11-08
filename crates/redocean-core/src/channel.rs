//! Decibel-scale Underwater Optical Wireless Communication (UOWC) channel physics.
//!
//! Implements closed-form decibel formulations for:
//! - Beer-Lambert extinction loss across Jerlov water regimes
//! - Geometric beam divergence path loss
//! - Pointing misalignment and spatial jitter loss
//! - Oceanic log-normal turbulence fading margin
//! - Electrical SNR and conditional framing Packet Error Rate (PER)

use serde::{Deserialize, Serialize};

/// Standard Jerlov seawater optical classifications.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WaterType {
    /// Deep pristine oceanic waters (c = 0.056 m^-1, 0.2432 dB/m).
    PureSea,
    /// Continental shelf clear waters (c = 0.151 m^-1, 0.6558 dB/m).
    ClearOcean,
    /// Coastal ocean waters with terrigenous runoff (c = 0.398 m^-1, 1.7285 dB/m).
    CoastalOcean,
    /// Estuarine and turbid harbor basins (c = 2.190 m^-1, 9.5110 dB/m).
    TurbidHarbor,
}

impl WaterType {
    /// Returns the optical beam extinction coefficient c (m^-1) at 532 nm.
    pub fn extinction_coefficient(&self) -> f64 {
        match self {
            WaterType::PureSea => 0.056,
            WaterType::ClearOcean => 0.151,
            WaterType::CoastalOcean => 0.398,
            WaterType::TurbidHarbor => 2.190,
        }
    }

    /// Returns the linear extinction slope in decibels per meter (dB/m):
    /// alpha_dB = 10 * log10(e) * c = 4.342944819 * c
    pub fn extinction_slope_db_per_m(&self) -> f64 {
        4.342944819032518 * self.extinction_coefficient()
    }
}

/// Optical link physical configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub water_type: WaterType,
    pub tx_power_w: f64,
    pub tx_power_dbm: f64,
    pub beam_divergence_deg: f64,
    pub aperture_diameter_m: f64,
    pub detector_responsivity_a_w: f64,
    pub receiver_bandwidth_hz: f64,
    pub load_resistance_ohms: f64,
    pub temperature_k: f64,
    pub noise_figure: f64,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            water_type: WaterType::ClearOcean,
            tx_power_w: 1.0,
            tx_power_dbm: 30.0,
            beam_divergence_deg: 20.0,
            aperture_diameter_m: 0.10, // 10 cm circular aperture
            detector_responsivity_a_w: 0.40,
            receiver_bandwidth_hz: 5e6, // 10 Mbps OOK (B_e = 5 MHz)
            load_resistance_ohms: 50.0,
            temperature_k: 290.0,
            noise_figure: 2.0,
        }
    }
}

/// Computes the Beer-Lambert extinction loss in decibels over distance d (m).
/// A_ext,dB(d) = 4.34294 * c * d [dB]
pub fn extinction_loss_db(water_type: WaterType, distance_m: f64) -> f64 {
    water_type.extinction_slope_db_per_m() * distance_m
}

/// Computes geometric beam divergence path loss in decibels over distance d (m).
/// For D_rx = 10 cm and theta_div = 20 deg, when d > 0.89 m:
/// A_geom,dB(d) = 10.94 + 20 * log10(d) [dB]
pub fn geometric_loss_db(aperture_diameter_m: f64, beam_div_deg: f64, distance_m: f64) -> f64 {
    let theta_div_rad = beam_div_deg.to_radians();
    let a_rx = std::f64::consts::PI * (aperture_diameter_m / 2.0).powi(2);
    let beam_footprint = std::f64::consts::PI * distance_m.powi(2) * (theta_div_rad / 2.0).tan().powi(2);
    
    if beam_footprint <= a_rx {
        0.0 // Near-field capture
    } else {
        10.0 * (beam_footprint / a_rx).log10()
    }
}

/// Computes pointing jitter attenuation in decibels:
/// A_p,dB(r) = 10*log10(1/A0) + (20/ln 10) * (r^2 / w_eq^2) [dB]
pub fn pointing_loss_db(radial_displacement_m: f64, w_eq_m: f64, a0_db: f64) -> f64 {
    a0_db + 8.685889638 * (radial_displacement_m / w_eq_m).powi(2)
}

/// Oceanic turbulence fade margin in decibels for target outage probability P_out.
/// M_turb,dB = 4.34294 * sqrt(2 * sigma_I^2) * erf_inv(1 - 2 * P_out)
pub fn turbulence_fade_margin_db(scintillation_index: f64) -> f64 {
    // For moderate turbulence (sigma_I^2 = 0.04) and P_out = 10^-3, erf_inv(0.998) ~ 2.185
    let erf_inv_target = 2.18512;
    4.342944819 * (2.0 * scintillation_index).sqrt() * erf_inv_target
}

/// Computes composite wireless channel path loss in decibels:
/// PL_UOWC,dB = A_ext + A_geom + A_point + A_turb
pub fn composite_path_loss_db(
    water: WaterType,
    distance_m: f64,
    aperture_diam_m: f64,
    beam_div_deg: f64,
    radial_disp_m: f64,
    scintillation_index: f64,
) -> f64 {
    let a_ext = extinction_loss_db(water, distance_m);
    let a_geom = geometric_loss_db(aperture_diam_m, beam_div_deg, distance_m);
    let a_point = pointing_loss_db(radial_disp_m, 0.50, 0.20);
    let a_turb = turbulence_fade_margin_db(scintillation_index);
    a_ext + a_geom + a_point + a_turb
}

/// Computes electrical SNR in decibels given received optical power (dBm).
pub fn electrical_snr_db(rx_power_dbm: f64, config: &ChannelConfig) -> f64 {
    // rx_power in Watts: 10^((P_dBm - 30) / 10)
    let p_rx_w = 10.0f64.powf((rx_power_dbm - 30.0) / 10.0);
    let q = 1.602176634e-19; // Elementary charge
    let k_b = 1.380649e-23;   // Boltzmann constant
    
    let i_dark = 5e-9;  // 5 nA dark current
    let i_bg = 10e-9;   // 10 nA solar ambient current
    let r = config.detector_responsivity_a_w;
    let b_e = config.receiver_bandwidth_hz;
    
    // Total noise variance: shot noise + thermal noise
    let sigma_shot_sq = 2.0 * q * (r * p_rx_w + i_dark + i_bg) * b_e;
    let sigma_thermal_sq = (4.0 * k_b * config.temperature_k * config.noise_figure * b_e) / config.load_resistance_ohms;
    let sigma_total_sq = sigma_shot_sq + sigma_thermal_sq;
    
    let signal_power_elec = (r * p_rx_w).powi(2);
    10.0 * (signal_power_elec / sigma_total_sq).log10()
}

/// Gaussian Q-function: Q(x) = 0.5 * erfc(x / sqrt(2))
pub fn q_function(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.5;
    }
    // High-accuracy approximation
    let t = 1.0 / (1.0 + 0.2316419 * x);
    let poly = t * (0.319381530 + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    let normal_pdf = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
    normal_pdf * poly
}

/// Conditional Bit Error Rate (BER) for OOK modulation with adaptive thresholding:
/// P_b = Q( 0.5 * sqrt(10^(SNR_dB / 10)) )
pub fn bit_error_rate(snr_db: f64) -> f64 {
    let snr_linear = 10.0f64.powf(snr_db / 10.0);
    let arg = 0.5 * snr_linear.sqrt();
    q_function(arg)
}

/// Framing Packet Error Rate: PER(L, d) = 1 - (1 - P_b)^L
pub fn packet_error_rate(frame_length_bytes: usize, snr_db: f64) -> f64 {
    let p_b = bit_error_rate(snr_db);
    let num_bits = (frame_length_bytes * 8) as f64;
    1.0 - (1.0 - p_b).powf(num_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extinction_slopes() {
        assert!((WaterType::ClearOcean.extinction_slope_db_per_m() - 0.6558).abs() < 1e-3);
        assert!((WaterType::CoastalOcean.extinction_slope_db_per_m() - 1.7286).abs() < 1e-3);
    }

    #[test]
    fn test_table_metrics_clear_ocean() {
        // Table II: Clear Ocean at d = 20 m
        let a_ext = extinction_loss_db(WaterType::ClearOcean, 20.0);
        assert!((a_ext - 13.12).abs() < 0.1);
        
        let a_geom = geometric_loss_db(0.10, 20.0, 20.0);
        assert!((a_geom - 36.96).abs() < 0.2);
    }

    #[test]
    fn test_per_optical_cliff() {
        // At operating point SNR ~ 18.1 dB (Clear Ocean at 20m)
        let per_lo_aka = packet_error_rate(36, 18.1);  // Flight 2 payload (36 B)
        let per_dtls = packet_error_rate(1200, 18.1);  // DTLS 1.3 frame (1200 B)
        
        assert!(per_lo_aka < 0.01, "LO-AKA PER should remain below 1%");
        assert!(per_dtls > 0.20, "DTLS should suffer higher PER (> 20%)");
    }
}
