use std::sync::Arc;
use chrono::TimeDelta;
use tokio::sync::Mutex;
use crate::models::client::Client;
use crate::models::server_connection_stats::ServerConnectionStats;

pub struct Config {
    active_clients: Vec<Client>,
    wait_clients: Vec<Client>,
    server_connection_stats: ServerConnectionStats,
}

impl Config {
    pub fn new() -> Arc<Mutex<Self>> {
        let active_clients = Vec::with_capacity(5);
        let wait_clients = Vec::new();
        let server_connection_stats = ServerConnectionStats::new();

        let config = Config {
            active_clients,
            wait_clients,
            server_connection_stats,
        };

        Arc::new(Mutex::new(config))
    }

    pub async fn add_client(&mut self, client: Client) -> Result<(), ()> {
        if self.active_clients.len() >= 5 {
            return Err(());
        }

        self.active_clients.push(client);
        Ok(())
    }

    pub async fn remove_client(&mut self, client: &Client) {
        let index = self.active_clients
            .iter()
            .position(|x| x == client)
            .unwrap();

        if self.active_clients[index].active_requests == 0 {
            self.server_connection_stats.add_count_handle_client().await;
            self.server_connection_stats.update_stats_session_time_client(&self.active_clients[index]).await;
            self.print_client_stats(&self.active_clients[index]).await;
            self.active_clients.remove(index);
            return;
        }

        self.active_clients[index].active_requests -= 1;
    }

    pub async fn exist_connect_client(&self, client: &Client) -> bool {
        self.active_clients
            .iter()
            .position(|x| x == client)
            .is_some()
    }

    pub async fn total_clients_connection(&self) -> usize {
        self.active_clients.len()
    }

    pub async fn add_client_wait(&mut self, client: Client) {
        self.wait_clients.push(client);
    }

    pub async fn remove_client_wait(&mut self, client: &Client) {
        let index = match self.wait_clients
            .iter()
            .position(|x| x == client) {
            Some(o) => o,
            None => return,
        };

        self.wait_clients.remove(index);
    }

    pub async fn exist_connect_client_wait(&self, client: &Client) -> bool {
        self.wait_clients
            .iter()
            .position(|x| x == client)
            .is_some()
    }

    pub async fn try_client_connect(&mut self, client: Client) -> bool {
        if self.exist_connect_client(&client).await {
            let index = self.active_clients
                .iter()
                .position(|x| x == &client)
                .unwrap();

            self.active_clients[index].active_requests += 1;
            return true;
        }

        if self.exist_connect_client_wait(&client).await && self.total_clients_connection().await >= 5 {
            return false;
        }

        if !self.exist_connect_client_wait(&client).await && self.total_clients_connection().await >= 5 {
            self.add_client_wait(client.clone()).await;
            return false;
        }

        if self.exist_connect_client_wait(&client).await && self.total_clients_connection().await < 5 {
            if self.add_client(client.clone()).await.is_ok() {
                self.remove_client_wait(&client).await;
                return true;
            }
        }

        self.add_client_wait(client.clone()).await;
        false
    }

    pub async fn add_count_request(&mut self, client: &Client) {
        let index = self.active_clients
            .iter()
            .position(|x| x == client)
            .unwrap();

        self.active_clients[index].add_count_request().await;
    }

    pub async fn update_stats_time_request(&mut self, client: &Client, time_delta: TimeDelta) {
        let index = self.active_clients
            .iter()
            .position(|x| x == client)
            .unwrap();

        self.active_clients[index].update_stats_time_request(time_delta).await;
    }

    pub async fn print_client_stats(&self, client: &Client) {
        let total_requests = client.total_count_request().await;
        let session_time = client.get_session_time().await.num_milliseconds();

        let connection_stats = &client.connection_stats;
        let min = connection_stats.min_processing_time;
        let max = connection_stats.max_processing_time;
        let avg = connection_stats.avg_processing_time;

        println!("Client {} is disconnected!", client.address);
        println!("Requests: {total_requests}");
        println!("Max request time: {max} ms");
        println!("Min request time: {min} ms");
        println!("Avg request time: {avg} ms");
        println!("Session time: {session_time} ms");
        println!("\n");
    }

    pub async fn print_server_stats(&self) {
        let wait_clients = self.wait_clients.len();
        println!("Server is shutdown!");
        println!("Clients is not handled: {wait_clients}");
        self.server_connection_stats.print_stats_report().await;
    }
}