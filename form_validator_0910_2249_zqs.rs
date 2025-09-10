use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, get};
use serde::Deserialize;
use serde_json::json;
use validator::{Validate, ValidationError, ValidationErrors};

// 定义一个结构体来表示表单数据
#[derive(Deserialize, Validate)]
struct FormData {
    username: String,
    email: String,
    // 可以根据需要添加更多的字段
}

// 创建一个函数来验证表单数据
fn validate_form_data(form_data: web::Json<FormData>) -> Result<HttpResponse, HttpResponse> {
    if form_data.validate().is_err() {
        Err(HttpResponse::BadRequest().json(json!({ 