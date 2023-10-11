use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponse {
}

#[post("/api/v1/test")]
async fn api_test() -> impl Responder {
    let response = EmptyResponse {
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> impl Responder {
    let response = EmptyResponse {};
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("4000".to_owned()).parse().unwrap();
    println!("Server listening: {}", port);
    HttpServer::new(|| App::new().service(api_test).default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
