use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use tokio::signal;
use crate::models::config::Config;
use crate::services::not_found::not_found;
use crate::services::work_service::work;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = std::env::var("HOST").expect("HOST is invalid!");
    let port = std::env::var("PORT").expect("PORT is invalid!")
        .parse::<u16>()
        .expect("PORT is not integer or big integer!");

    let config = web::Data::new(Config::new());
    let clone_config = Arc::clone(&config);

    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        clone_config.lock().await.print_server_stats().await;
        std::process::exit(0x0000);
    });

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(work)
            .default_service(
                web::route().to(not_found)
            )
    })
        .bind((host, port))?
        .run()
        .await
}

mod services;
mod models;
mod utils;