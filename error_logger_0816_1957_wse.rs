use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::io;

#[derive(Debug, Clone)]
struct ErrorLogger;

impl actix_web::error::Error for ErrorLogger {
    fn as_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

async fn log_error(item: web::Json<ErrorLogger>) -> impl Responder {
    // Log the error
    println!("Error logged: {:?}