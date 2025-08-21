 * Features:
 * - Addition
 * - Subtraction
 * - Multiplication
 * - Division
 *
 * This service provides a simple REST API to perform basic arithmetic operations.
 */

use actix_web::{
    web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use thiserror::Error;

// Define a struct for the API request payload
#[derive(Serialize, Deserialize)]
struct MathOperation<T> {
    a: T,
    b: T,
    operation: String,
}

// Define a struct for the API response payload
#[derive(Serialize, Deserialize)]
struct MathResult<T> {
    result: Option<T>,
    error: Option<String>,
}

// Define error types
#[derive(Error, Debug)]
pub enum MathError {
    #[error("Invalid operation")]
    InvalidOperation,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid input")]
    InvalidInput,
}

// Implement error conversion for Actix
impl actix_web::error::ResponseError for MathError {
    fn error_response(&self) -> actix_web::error::ErrorResponseBody {
        match *self {
            MathError::InvalidOperation =>
                HttpResponse::BadRequest().json(MathResult::<String> {
                    result: None,
                    error: Some("Invalid operation".to_string()),
                }),
            MathError::DivisionByZero =>
                HttpResponse::BadRequest().json(MathResult::<String> {
                    result: None,
                    error: Some("Division by zero".to_string()),
                }),
            MathError::InvalidInput =>
                HttpResponse::BadRequest().json(MathResult::<String> {
                    result: None,
                    error: Some("Invalid input".to_string()),
                }),
        }
    }
}

// Implement the API handler
async fn calculate<T>(item: web::Json<MathOperation<T>>) -> impl Responder
where
    T: FromStr + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
    <T as FromStr>::Err: std::fmt::Display,
{
    let MathOperation { a, b, operation } = item.into_inner();

    let result = match operation.as_str() {
        "add" => a.add(b),
        "subtract" => a.sub(b),
        "multiply" => a.mul(b),
        "divide" => {
            if b == T::from(0).unwrap() {
                Err(MathError::DivisionByZero)
            } else {
                a.div(b)
            }
        },
        _ => return Err(MathError::InvalidOperation),
    };

    match result {
        Ok(val) => HttpResponse::Ok().json(MathResult { result: Some(val), error: None }),
        Err(e) => Err(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/add", web::post().to(calculate::<i32>))
            .route("/subtract", web::post().to(calculate::<i32>))
            .route("/multiply", web::post().to(calculate::<i32>))
            .route("/divide", web::post().to(calculate::<i32>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
