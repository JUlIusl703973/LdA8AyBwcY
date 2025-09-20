use actix_web::{web, App, HttpServer, Responder, HttpResponse, error::ErrorBadRequest};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;
use validator::Validation;

// 定义一个表单数据结构体，用于验证
#[derive(Debug, Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    name: String,
    #[validate(email(message = "Email is not valid"))]
    email: String,
    #[validate(custom = "is_positive", message = "Age must be positive"))]
    age: u32,
}

// 实现自定义验证规则
fn is_positive(value: &str) -> Result<bool, validator::ValidationError> {
    match value.parse::<u32>() {
        Ok(n) if n > 0 => Ok(true),
        _ => Err(validator::ValidationError::new("