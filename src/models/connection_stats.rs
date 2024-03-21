use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub request_count: u64,
    pub max_processing_time: u128,
    pub min_processing_time: u128,
    pub avg_processing_time: u128,
    pub session_time: Instant,
}

impl ConnectionStats {
    pub fn new() -> Self {
        let request_count = 0u64;
        let max_processing_time = 0u128;
        let min_processing_time = 0u128;
        let avg_processing_time = 0u128;
        let session_time = Instant::now();

        ConnectionStats {
            request_count,
            max_processing_time,
            min_processing_time,
            avg_processing_time,
            session_time,
        }
    }
}