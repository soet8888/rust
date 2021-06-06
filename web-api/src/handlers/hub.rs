use super::vo::Hub;
use actix_web::{get, HttpResponse, Responder};
use sysinfo::SystemExt;
#[get("/hb")]
pub async fn hub() -> impl Responder {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    let h = Hub {
        status: format!("{}", "Ok"),
        version: format!("{}", "v0.1.0"),
        total_mem: format!("{}", system.get_total_memory()),
        free_mem: format!("{}", system.get_free_memory()),
        used_mem: format!("{}", system.get_used_memory()),
    };
    HttpResponse::Ok().json(h)
}
