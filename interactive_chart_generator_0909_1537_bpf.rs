use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;
use plotters::prelude::*;

// 定义请求数据结构
#[derive(Serialize, Deserialize)]
struct ChartRequest {
    title: String,
    x_label: String,
    y_label: String,
    data: Vec<(f64, f64)>,
}

// 定义响应数据结构
#[derive(Serialize, Deserialize)]
struct ChartResponse {
    filename: String,
    // 其他响应字段...
}

// 定义错误处理
#[derive(Debug)]
enum ChartError {
    InvalidData,
    PlottingError,
    // 其他错误类型...
}

// 实现错误处理
impl From<ChartError> for HttpResponse {
    fn from(error: ChartError) -> Self {
        match error {
            ChartError::InvalidData => HttpResponse::BadRequest().json(json!{"error": "Invalid data provided"}