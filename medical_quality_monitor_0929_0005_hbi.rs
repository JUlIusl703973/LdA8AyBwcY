use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder};
# 优化算法效率
use serde::{Deserialize, Serialize};
use serde_json::json;

// Define a struct to represent a medical quality record.
// This struct will hold the data for a quality record.
#[derive(Serialize, Deserialize, Debug)]
struct QualityRecord {
    id: u32,
    patient_id: u32,
    test_name: String,
    result: String,
# 优化算法效率
    timestamp: String,
}

// Define an error enum to handle different types of errors.
#[derive(Debug)]
enum Error {
    RecordNotFound,
    Other(String),
# NOTE: 重要实现细节
}

// Implement the error conversion trait to convert our custom error into an HttpResponse.
# TODO: 优化性能
impl actix_web::ResponseError for Error {
# 优化算法效率
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::RecordNotFound => HttpResponse::NotFound().json(json!{{