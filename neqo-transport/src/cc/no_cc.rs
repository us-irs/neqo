use std::fmt::Display;

use super::CongestionControl;

#[derive(Debug)]
pub struct NoCc {
    pmtud: crate::Pmtud,
}

impl NoCc {
    pub const fn new(pmtud: crate::Pmtud) -> Self {
        Self { pmtud }
    }
}

impl Display for NoCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoCc",)?;
        Ok(())
    }
}

impl CongestionControl for NoCc {
    fn set_qlog(&mut self, _qlog: neqo_common::qlog::NeqoQlog) {}

    fn cwnd(&self) -> usize {
        0
    }

    fn bytes_in_flight(&self) -> usize {
        0
    }

    fn cwnd_avail(&self) -> usize {
        0
    }

    fn cwnd_min(&self) -> usize {
        0
    }

    #[cfg(test)]
    fn cwnd_initial(&self) -> usize {
        0
    }

    fn pmtud(&self) -> &crate::Pmtud {
        &self.pmtud
    }

    fn pmtud_mut(&mut self) -> &mut crate::Pmtud {
        &mut self.pmtud
    }

    fn on_packets_acked(
        &mut self,
        _acked_pkts: &[crate::recovery::SentPacket],
        _rtt_est: &crate::rtt::RttEstimate,
        _now: std::time::Instant,
    ) {
    }

    fn on_packets_lost(
        &mut self,
        _first_rtt_sample_time: Option<std::time::Instant>,
        _prev_largest_acked_sent: Option<std::time::Instant>,
        _pto: std::time::Duration,
        _lost_packets: &[crate::recovery::SentPacket],
    ) -> bool {
        false
    }

    fn on_ecn_ce_received(&mut self, _largest_acked_pkt: &crate::recovery::SentPacket) -> bool {
        false
    }

    fn recovery_packet(&self) -> bool {
        false
    }

    fn discard(&mut self, _pkt: &crate::recovery::SentPacket) {}

    fn on_packet_sent(&mut self, _pkt: &crate::recovery::SentPacket) {}

    fn discard_in_flight(&mut self) {}
}
