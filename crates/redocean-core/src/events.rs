//! Discrete-event simulation queue and telemetry event records.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub timestamp_sec: f64,
    pub event_id: u64,
    pub payload_name: String,
}

impl PartialEq for SimEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp_sec == other.timestamp_sec && self.event_id == other.event_id
    }
}

impl Eq for SimEvent {}

impl PartialOrd for SimEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap (earliest event first)
        other.timestamp_sec.partial_cmp(&self.timestamp_sec).unwrap_or(Ordering::Equal)
    }
}

pub struct EventLoop {
    queue: BinaryHeap<SimEvent>,
    pub current_time_sec: f64,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            current_time_sec: 0.0,
        }
    }

    pub fn schedule(&mut self, timestamp_sec: f64, event_id: u64, payload_name: &str) {
        self.queue.push(SimEvent {
            timestamp_sec,
            event_id,
            payload_name: payload_name.to_string(),
        });
    }

    pub fn step(&mut self) -> Option<SimEvent> {
        if let Some(event) = self.queue.pop() {
            self.current_time_sec = event.timestamp_sec;
            Some(event)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_loop_chronological_order() {
        let mut ev_loop = EventLoop::new();
        ev_loop.schedule(2.5, 2, "Flight2");
        ev_loop.schedule(1.0, 1, "Flight1");
        ev_loop.schedule(3.2, 3, "Flight3");

        let e1 = ev_loop.step().unwrap();
        assert_eq!(e1.event_id, 1);
        assert_eq!(e1.timestamp_sec, 1.0);

        let e2 = ev_loop.step().unwrap();
        assert_eq!(e2.event_id, 2);

        let e3 = ev_loop.step().unwrap();
        assert_eq!(e3.event_id, 3);
        assert!(ev_loop.is_empty());
    }
}

