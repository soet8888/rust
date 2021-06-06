use super::vo::ClientResponse;
use actix_web::{get, HttpResponse, Responder};
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(ClientResponse {
        status: format!("{}", "Ok"),
        data: format!("{}", ""),
        message: format!("{}", "from main route...."),
    })
}
