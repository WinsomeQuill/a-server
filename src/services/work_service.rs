use std::sync::Arc;
use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use actix_web::web::Payload;
use tokio::sync::Mutex;
use tokio::time::Instant;
use crate::models::client::Client;
use crate::models::config::Config;
use crate::models::dto::calculator_dto::CalculatorDto;
use crate::models::response::Response;
use crate::services::{convert_body_to_struct, read_body_bytes};
use crate::utils::utils::{calculating, generate_delay};

#[post("/work")]
pub async fn work(config: web::Data<Arc<Mutex<Config>>>, req: HttpRequest, mut payload: Payload) -> impl Responder {
    let client = Client::new(req.peer_addr().unwrap().ip().to_string());

    loop {
        if config.clone().lock().await.try_client_connect(client.clone()).await {
            break;
        }
    }

    let start_time = Instant::now();

    config.lock().await.add_count_request(&client).await;

    let body = match read_body_bytes(&mut payload).await {
        Ok(o) => o,
        Err(_) => return HttpResponse::BadRequest().json(
            Response::error("Request overflow!")
        )
    };

    let calculator = match convert_body_to_struct::<CalculatorDto>(&body).await {
        Ok(o) => o,
        Err(_) => return HttpResponse::BadRequest().json(
            Response::error("Bad request!")
        )
    };

    let result = match calculating(calculator).await {
        Ok(o) => o,
        Err(e) => return HttpResponse::BadRequest().json(
            Response::error(e)
        )
    };

    generate_delay().await;

    let total_time = start_time.elapsed();
    config.lock().await.update_stats_time_request(&client, total_time).await;
    config.lock().await.close_request_client(&client).await;

    HttpResponse::Ok().json(
        Response::success(format!("Your result is {result}!"))
    )
}