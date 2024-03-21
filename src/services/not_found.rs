use std::sync::Arc;
use actix_web::{HttpRequest, HttpResponse, web};
use tokio::sync::Mutex;
use tokio::time::Instant;
use crate::models::client::Client;
use crate::models::config::Config;
use crate::models::response::Response;
use crate::utils::utils::generate_delay;

pub async fn not_found(config: web::Data<Arc<Mutex<Config>>>, req: HttpRequest) -> HttpResponse {
    let client = Client::new(req.peer_addr().unwrap().ip().to_string());

    loop {
        if config.clone().lock().await.try_client_connect(client.clone()).await {
            break;
        }
    }

    let start_time = Instant::now();

    config.lock().await.add_count_request(&client).await;

    generate_delay().await;

    let total_time = start_time.elapsed();
    config.lock().await.update_stats_time_request(&client, total_time).await;
    config.lock().await.close_request_client(&client).await;

    HttpResponse::NotFound().json(
        Response::error("Page not found!")
    )
}