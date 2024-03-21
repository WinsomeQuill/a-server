use crate::models::client::Client;

pub struct ServerConnectionStats {
    total_handled_clients: u64,
    min_session_time_client: u128,
    max_session_time_client: u128,
    avg_session_time_client: u128,
}

impl ServerConnectionStats {
    pub fn new() -> Self {
        let total_handled_clients = 0u64;
        let min_session_time_client = 0u128;
        let max_session_time_client = 0u128;
        let avg_session_time_client = 0u128;

        ServerConnectionStats {
            total_handled_clients,
            min_session_time_client,
            max_session_time_client,
            avg_session_time_client,
        }
    }

    pub async fn add_count_handle_client(&mut self) {
        self.total_handled_clients += 1;
    }

    pub async fn update_stats_session_time_client(&mut self, client: &Client) {
        let milliseconds = client.get_session_time().await.as_millis();

        if self.min_session_time_client == 0 {
            self.min_session_time_client = milliseconds;
        }

        if self.min_session_time_client > milliseconds {
            self.min_session_time_client = milliseconds;
        }

        if self.max_session_time_client < milliseconds {
            self.max_session_time_client = milliseconds;
        }

        self.avg_session_time_client = (self.max_session_time_client + self.min_session_time_client) / 2;
    }

    pub async fn print_stats_report(&self) {
        let min = self.min_session_time_client;
        let max = self.max_session_time_client;
        let avg = self.avg_session_time_client;
        let handles_clients = self.total_handled_clients;

        println!("Handles clients: {handles_clients}");
        println!("Max session time: {max}");
        println!("Min session time: {min}");
        println!("Avg session time: {avg}");
    }
}