use super::vo::ClientResponse;
use actix_web::{HttpResponse, Responder};
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().json(ClientResponse {
        status: format!("{}", "Ok"),
        data: format!("{}", ""),
        message: format!("{}", "from hey route..."),
    })
}
