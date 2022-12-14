use actix_web::{get, Responder, HttpResponse};
use crate::models::todo_items_model::Status;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
        .json( Status { status: "Ok".to_string() })
}
