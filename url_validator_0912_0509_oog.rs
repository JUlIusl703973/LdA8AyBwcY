use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responde};
use url::Url;
use std::io;
use thiserror::Error;

// 定义一个错误类型，用于处理URL验证过程中可能发生的错误
#[derive(Error, Debug)]
pub enum UrlValidationError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
}

/// 验证URL是否有效
/// 
/// 这个函数尝试解析提供的URL字符串，并返回一个结果。
/// 如果URL无效，返回相应的错误。
/// 
/// # 参数
/// * `url_str`: 要验证的URL字符串
/// 
/// # 返回值
/// * `Result<Url, UrlValidationError>`: 成功时返回URL结构体，失败时返回错误
fn validate_url(url_str: &str) -> Result<Url, UrlValidationError> {
    Url::parse(url_str).map_err(|e| UrlValidationError::InvalidUrl(e.to_string()))
}

/// 处理GET请求的函数，用于验证URL的有效性
/// 
/// 这个函数接收一个URL路径参数，尝试验证其有效性，并返回结果。
/// 
/// # 参数
/// * `url_path`: 要验证的URL路径参数
/// 
/// # 返回值
/// * `impl Responder`: 返回响应结果
#[get("/validate/{url_path}")]
async fn validate_url_handler(url_path: web::Path<String>) -> impl Responder {
    match validate_url(&url_path.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("URL is valid"),
        Err(e) => HttpResponse::BadRequest().json(format!("Invalid URL: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(validate_url_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
