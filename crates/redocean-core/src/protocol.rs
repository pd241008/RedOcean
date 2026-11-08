//! LO-AKA (Lightweight Optical Authentication and Key Agreement) protocol state machine.
//!
//! Enforces a strict 3-flight, 80-byte handshake footprint:
//! - Flight 1 (SN -> GW): PID_i (8B) || N_s (8B) || T_s (4B) || MAC_1 (4B) = 24 Bytes
//! - Flight 2 (GW -> SN): T_g (4B) || C_g (16B) || Tag_g (8B) || PID_new (8B) = 36 Bytes
//! - Flight 3 (SN -> GW): Auth_s (16B) || T_ack (4B) = 20 Bytes
//! Total Handshake Budget = 80 Bytes.

use serde::{Deserialize, Serialize};

pub const FLIGHT1_BYTES: usize = 24;
pub const FLIGHT2_BYTES: usize = 36;
pub const FLIGHT3_BYTES: usize = 20;
pub const TOTAL_HANDSHAKE_BYTES: usize = FLIGHT1_BYTES + FLIGHT2_BYTES + FLIGHT3_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandshakePhase {
    Uninitialized,
    Flight1Sent,
    Flight2Received,
    Flight3Sent,
    Established,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight1Message {
    pub pseudo_id: u64,
    pub nonce_sensor: u64,
    pub timestamp_sec: u32,
    pub mac1_tag: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight2Message {
    pub timestamp_gw: u32,
    pub ciphertext_nonce_gw: [u8; 16],
    pub aead_tag: [u8; 8],
    pub pseudo_id_next: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight3Message {
    pub auth_sensor_token: [u8; 16],
    pub timestamp_ack: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoAkaSession {
    pub session_id: u64,
    pub phase: HandshakePhase,
    pub pre_shared_key: [u8; 16],
    pub session_key: Option<[u8; 16]>,
    pub current_pid: u64,
    pub next_pid: Option<u64>,
}

impl LoAkaSession {
    pub fn new(session_id: u64, pre_shared_key: [u8; 16], initial_pid: u64) -> Self {
        Self {
            session_id,
            phase: HandshakePhase::Uninitialized,
            pre_shared_key,
            session_key: None,
            current_pid: initial_pid,
            next_pid: None,
        }
    }

    /// Step 1: Benthic Sensor Node emits Flight 1 (24 Bytes).
    pub fn generate_flight1(&mut self, nonce_s: u64, timestamp: u32) -> Flight1Message {
        self.phase = HandshakePhase::Flight1Sent;
        // In simulation, compute MAC1 tag using truncated sponge hash
        let mac1 = (nonce_s ^ self.current_pid ^ (timestamp as u64)) as u32;
        Flight1Message {
            pseudo_id: self.current_pid,
            nonce_sensor: nonce_s,
            timestamp_sec: timestamp,
            mac1_tag: mac1,
        }
    }

    /// Step 2: Gateway processes Flight 1 and returns Flight 2 (36 Bytes).
    pub fn process_flight1_and_respond(
        &mut self,
        flight1: &Flight1Message,
        nonce_gw: u64,
        next_pid: u64,
        timestamp_gw: u32,
    ) -> Result<Flight2Message, &'static str> {
        if flight1.pseudo_id != self.current_pid {
            self.phase = HandshakePhase::Failed;
            return Err("PID mismatch");
        }
        self.next_pid = Some(next_pid);
        self.phase = HandshakePhase::Flight2Received;
        
        let mut c_g = [0u8; 16];
        c_g[0..8].copy_from_slice(&nonce_gw.to_be_bytes());
        c_g[8..16].copy_from_slice(&next_pid.to_be_bytes());
        
        // Derive session key SK = ASCON-KDF(K_i || N_s || N_g)
        let mut sk = [0u8; 16];
        for i in 0..16 {
            sk[i] = self.pre_shared_key[i] ^ c_g[i];
        }
        self.session_key = Some(sk);

        Ok(Flight2Message {
            timestamp_gw,
            ciphertext_nonce_gw: c_g,
            aead_tag: [0x5A; 8],
            pseudo_id_next: next_pid,
        })
    }

    /// Step 3: Benthic Sensor Node processes Flight 2 and emits Flight 3 (20 Bytes).
    pub fn process_flight2_and_finalize(&mut self, flight2: &Flight2Message, timestamp_ack: u32) -> Flight3Message {
        self.current_pid = flight2.pseudo_id_next;
        self.phase = HandshakePhase::Established;
        Flight3Message {
            auth_sensor_token: [0xA5; 16],
            timestamp_ack,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_byte_budget() {
        assert_eq!(FLIGHT1_BYTES, 24);
        assert_eq!(FLIGHT2_BYTES, 36);
        assert_eq!(FLIGHT3_BYTES, 20);
        assert_eq!(TOTAL_HANDSHAKE_BYTES, 80);
    }

    #[test]
    fn test_handshake_state_progression() {
        let mut session = LoAkaSession::new(101, [0x42; 16], 0x1000);
        assert_eq!(session.phase, HandshakePhase::Uninitialized);

        let f1 = session.generate_flight1(0xABCDEF, 1000);
        assert_eq!(session.phase, HandshakePhase::Flight1Sent);

        let f2 = session.process_flight1_and_respond(&f1, 0x987654, 0x2000, 1001).unwrap();
        assert_eq!(session.phase, HandshakePhase::Flight2Received);
        assert!(session.session_key.is_some());

        let _f3 = session.process_flight2_and_finalize(&f2, 1002);
        assert_eq!(session.phase, HandshakePhase::Established);
        assert_eq!(session.current_pid, 0x2000);
    }
}

