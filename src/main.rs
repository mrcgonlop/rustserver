use std::sync::Mutex;

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;


#[get("/hello")]
async fn hello() -> impl Responder {
    info!("Sending a String.");
    "Hallo Welt"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(Mutex::new(0))
            .service(hello)
            .service(Files::new("/", "./diox/dist/").index_file("index.html"))
            .default_service(
                web::route().to(|| HttpResponse::Found().header("Location", "/").finish()),
            )
    })
    .bind("192.168.0.22:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
