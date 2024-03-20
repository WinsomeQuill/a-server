use chrono::{TimeDelta, Utc};
use crate::models::connection_stats::ConnectionStats;

#[derive(Debug, Clone)]
pub struct Client {
    pub address: String,
    connection_stats: ConnectionStats,
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

    pub async fn get_session_time(&self) -> TimeDelta {
        Utc::now() - self.connection_stats.session_time
    }

    pub async fn update_stats_time_request(&mut self, time_delta: TimeDelta) {
        let milliseconds = time_delta.num_milliseconds();
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

    pub async fn print_stats_report(&self) {
        let connection_stats = &self.connection_stats;
        let min = connection_stats.min_processing_time;
        let max = connection_stats.max_processing_time;
        let avg = connection_stats.avg_processing_time;

        println!("Max request time: {max}");
        println!("Min request time: {min}");
        println!("Avg request time: {avg}");
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}