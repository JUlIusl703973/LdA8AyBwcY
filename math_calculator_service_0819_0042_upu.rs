use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fmt;

// 定义数学操作的结构体
#[derive(Deserialize)]
pub struct MathOperation {
    operand1: f64,
    operand2: f64,
    operation: String,
}

// 定义错误结构体，用于错误响应
#[derive(Debug, fmt::Display)]
pub enum MathError {
    InvalidOperation,
    DivisionByZero,
}

// 实现错误的转换为响应体
impl actix_web::error::ResponseError for MathError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            MathError::InvalidOperation => actix_web::http::StatusCode::BAD_REQUEST,
            MathError::DivisionByZero => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/plain")
            .body(self.to_string())
    }
}

// 定义数学计算的服务
pub async fn math_calculator(params: web::Json<MathOperation>) -> impl Responder {
    let result = match params.operation.as_str() {
        "add" => Ok(params.operand1 + params.operand2),
        "subtract" => Ok(params.operand1 - params.operand2),
        "multiply" => Ok(params.operand1 * params.operand2),
        "divide" => if params.operand2 == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(params.operand1 / params.operand2)
        },
        _ => Err(MathError::InvalidOperation),
    };
    match result {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => e.error_response(),
    }
}

// 主函数，设置服务并启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(math_calculator)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
