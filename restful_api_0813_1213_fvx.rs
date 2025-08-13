use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, Result, get, post};
use serde::Deserialize;
use serde_json::{json};

// 定义请求数据结构
#[derive(Deserialize)]
struct User {
    name: String,
    age: u32,
}

// API的错误响应结构体
#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    Other(String),
}

// 实现`Responder` trait，将错误转换为HTTP响应
impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().json(json!({"error": "Resource not found"})),
            ApiError::InvalidInput(msg) => HttpResponse::BadRequest().json(json!({"error": "Invalid input", "message": msg})),
            ApiError::Other(msg) => HttpResponse::InternalServerError().json(json!({"error": "Internal server error", "message": msg})),
        }
    }
}

// 用户信息的GET请求处理器
#[get("/users/{id}")]
async fn user_info(id: web::Path<u32>) -> Result<impl Responder, ApiError> {
    match get_user_by_id(id.into_inner()) {
        Some(user) => Ok(HttpResponse::Ok().json(json!({"name": user.name, "age": user.age}))),
        None => Err(ApiError::NotFound),
    }
}

// 用户信息的POST请求处理器
#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    if user.age < 18 {
        return HttpResponse::BadRequest().json(json!({"error": "User age must be at least 18"}));
    }
    // 假设保存用户信息到数据库
    save_user(user.into_inner());
    HttpResponse::Ok().json(json!({"message": "User created successfully"}))
}

// 假设的用户数据库
fn get_user_by_id(id: u32) -> Option<User> {
    // 这里只是一个示例，实际应用中应该查询数据库
    Some(User {
        name: "John Doe".to_string(),
        age: 30,
    })
}

// 假设的用户保存函数
fn save_user(user: User) {
    // 这里只是一个示例，实际应用中应该将用户信息保存到数据库
    println!("Saving user: {} with age {}", user.name, user.age);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_info)
            .service(create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
