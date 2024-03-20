use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub request_count: u64,
    pub max_processing_time: i64,
    pub min_processing_time: i64,
    pub avg_processing_time: i64,
    pub session_time: DateTime<Utc>,
}

impl ConnectionStats {
    pub fn new() -> Self {
        let request_count = 0u64;
        let max_processing_time = 0i64;
        let min_processing_time = 0i64;
        let avg_processing_time = 0i64;
        let session_time = Utc::now();

        ConnectionStats {
            request_count,
            max_processing_time,
            min_processing_time,
            avg_processing_time,
            session_time,
        }
    }
}