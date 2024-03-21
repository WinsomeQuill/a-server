use tokio::time::Duration;
use crate::models::connection_stats::ConnectionStats;

#[derive(Debug, Clone)]
pub struct Client {
    pub address: String,
    pub connection_stats: ConnectionStats,
    pub active_requests: u32,
}

impl Client {
    pub fn new(address: String) -> Self {
        let connection_stats = ConnectionStats::new();
        let active_requests = 0;

        Client {
            address,
            connection_stats,
            active_requests,
        }
    }

    pub async fn add_count_request(&mut self) {
        self.connection_stats.request_count += 1;
    }

    pub async fn total_count_request(&self) -> u64 {
        self.connection_stats.request_count
    }

    pub async fn get_session_time(&self) -> Duration {
        self.connection_stats.session_time.elapsed()
    }

    pub async fn update_stats_time_request(&mut self, duration: Duration) {
        let milliseconds = duration.as_millis();
        let connection_stats = &mut self.connection_stats;

        if connection_stats.min_processing_time == 0 {
            connection_stats.min_processing_time = milliseconds;
        }

        if connection_stats.min_processing_time > milliseconds {
            connection_stats.min_processing_time = milliseconds;
        }

        if connection_stats.max_processing_time < milliseconds {
            connection_stats.max_processing_time = milliseconds;
        }

        self.connection_stats.avg_processing_time = (connection_stats.max_processing_time + connection_stats.min_processing_time) / 2;
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}