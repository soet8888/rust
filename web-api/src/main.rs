mod handlers;
use actix_web::{web, App, HttpServer};
use handlers::{echo::echo, hello::hello, hub::hub, manual_hello::manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server listening on localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(hub)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
