use actix_web::{post, HttpResponse, Responder};

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().json(req_body)
}
