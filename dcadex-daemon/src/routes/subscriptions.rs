use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    name: String,
    email: String,
}
pub async fn subscribe(_form: web::Form<Info>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
