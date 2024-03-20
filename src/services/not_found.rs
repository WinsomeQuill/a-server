use actix_web::HttpResponse;
use crate::models::response::Response;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(
        Response::error("Page not found!")
    )
}