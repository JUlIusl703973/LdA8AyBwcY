use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use serde::Deserialize;
# NOTE: 重要实现细节
use serde_json::{json, Value};
use validator::{Validate, ValidationError, Validator};

/// 定义表单数据结构体，使用 `#[derive(Deserialize)]` 来自动从JSON中解析
# TODO: 优化性能
#[derive(Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    name: String,
    
    #[validate(email(message = "Invalid email"))]
    email: String,
    
    #[validate(custom = "validate_age", message = "Age must be greater than 18"))]
    age: u32,
# 增强安全性
}

/// 实现自定义验证函数
fn validate_age(field: &u32) -> Result<(), ValidationError> {
    if *field <= 18 {
# 添加错误处理
        Err(ValidationError::new("Age must be greater than 18"))
    } else {
# 优化算法效率
        Ok(())
    }
# 改进用户体验
}

/// 表单数据验证器处理函数
#[post("/validate")]
async fn validate_form(data: web::Json<FormData>) -> impl Responder {
    if let Err(err) = data.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": err.to_string() }));
    }
    
    HttpResponse::Ok().json(json!({ "message": "Form data is valid" }))
# 扩展功能模块
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# NOTE: 重要实现细节
    HttpServer::new(|| {
        App::new()
            .service(validate_form)
    })
    .bind("127.0.0.1:8080")?
    .run()
# 扩展功能模块
    .await
}