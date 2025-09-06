use actix_web::{web, App, HttpServer, Responder, get};
use actix_web::dev::ServiceRequest;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use validator::{Validate, ValidationError, Validator};

// 定义表单数据结构体，使用serde进行序列化和反序列化
#[derive(Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 1, max = 100))]
    username: String, // 用户名字段，长度限制在1到100字符之间

    #[validate(email)]
    email: String, // 邮箱字段，需要符合邮箱格式

    #[validate(custom = "validate_password")]
    password: String, // 密码字段，需要符合自定义的密码验证函数
}

// 自定义密码验证函数
fn validate_password(value: &str) -> Result<(), ValidationError> {
    if value.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    if value.is_ascii() {
        return Err(ValidationError::new("password_must_have_non_ascii_chars"));
    }
    Ok(())
}

// 表单验证处理器
async fn validate_form(data: web::Json<FormData>) -> impl Responder {
    if let Err(e) = data.validate() {
        return Err(ErrorBadRequest(e.to_string()));
    }

    // 验证成功，返回成功响应
    json!({