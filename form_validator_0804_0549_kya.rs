use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;
use validator::validation::ValidationError;

// 定义表单数据结构
#[derive(Debug, Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 1, max = 20))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(range(min = 1, max = 100))]
    age: u8,
}

// 表单数据验证器
async fn validate_form(data: web::Json<FormData>) -> impl Responder {
    if let Err(e) = data.validate() {
        return HttpResponse::BadRequest().json(json!({
            "errors": ValidationError::to_string(&e),
        }));
    }
    // 业务逻辑处理
    // ...
    
    HttpResponse::Ok().json(json!({
        "message": "Form data is valid",
        "data": data.into_inner(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/validate", web::post().to(validate_form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
