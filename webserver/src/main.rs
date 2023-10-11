use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

use wasm_runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponse {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResponse {
    msg: u32,
}

#[post("/api/v1/test")]
async fn api_test() -> impl Responder {
    let response = EmptyResponse {
    };
    HttpResponse::Ok().json(response)
}

#[post("/api/v1/api1")]
async fn api_api1() -> impl Responder {
    let wasm_file_path = "./wasm/image_utils_wasm.wasm";
    let fn_name = "api1";
    let msg = wasm_runner::execute_wasm(wasm_file_path, fn_name);
    let response = CustomResponse {
        msg: msg
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
    HttpServer::new(|| App::new()
        .service(api_test)
        .service(api_api1)
        .default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
