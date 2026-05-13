use anyhow::{Context, Result};
use aya::maps::Array;
use aya::Bpf;

const SOCKET_DROP_COUNT_MAP: &str = "SOCKET_DROP_COUNTS";

#[derive(Debug)]
pub struct SocketDropStats {
    counts: [u64; 6],
}

impl SocketDropStats {
    pub fn from_bpf(bpf: &mut Bpf) -> Result<Self> {
        let mut counts = [0u64; 6];

        // Get the BPF map
        let map: Array<_, u64> = bpf
            .take_map(SOCKET_DROP_COUNT_MAP)
            .context("Failed to get socket_drop_counts map")?
            .try_into()
            .context("Failed to convert to Array map")?;

        // Read each entry from the map (flags = 0 for no special behavior)
        for i in 0..6 {
            if let Ok(value) = map.get(&i, 0) {
                counts[i as usize] = value;
            }
        }

        Ok(SocketDropStats { counts })
    }

    pub fn print(&self) {
        let reasons = [
            "NO_SOCKET",
            "SOCKET_CLOSE",
            "SOCKET_FILTER",
            "SOCKET_RCVBUFF",
            "SOCKET_BACKLOG",
            "PACKET_SOCK_ERROR",
        ];

        println!("Socket drop statistics:");
        let mut total = 0u64;

        for (i, reason) in reasons.iter().enumerate() {
            let count = self.counts[i];
            total += count;
            println!("- {}: {}", reason, count);
        }

        println!("- TOTAL: {}", total);
    }

    /// Get count for a specific reason
    #[allow(dead_code)]
    pub fn get(&self, idx: usize) -> u64 {
        if idx < 6 {
            self.counts[idx]
        } else {
            0
        }
    }

    /// Get total count
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.counts.iter().sum()
    }
}
