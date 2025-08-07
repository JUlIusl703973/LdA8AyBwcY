use actix_web::{web, App, HttpServer, HttpResponse, Responder, post, get};
use serde::Deserialize;
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义用户结构体，包含用户名和密码
#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

// 用户登录请求结构体
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// 模拟数据库，存储用户信息
lazy_static! {
    static ref DATABASE: Mutex<HashMap<String, User>> = {
        let mut m = HashMap::new();
        m.insert("admin".to_string(), User {
            username: "admin".to_string(),
            password: "password".to_string(),
        });
        Mutex::new(m)
    };
}

// 用户服务结构体
struct UserService;

#[post("/login")]
async fn login(user: web::Json<LoginRequest>) -> impl Responder {
    // 从模拟数据库中获取用户信息
    let db = DATABASE.lock().unwrap();
    match db.get(&user.username) {
        Some(user_data) => {
            // 验证密码
            if user_data.password == user.password {
                HttpResponse::Ok().json(json!{"message": "登录成功", "username": user.username})
            } else {
                HttpResponse::Unauthorized().json(json!{"message": "密码错误"})
            }
        }
        None => {
            HttpResponse::NotFound().json(json!{"message": "用户不存在"})
        }
    }
}

#[get("/users")]
async fn list_users() -> impl Responder {
    let db = DATABASE.lock().unwrap();
    HttpResponse::Ok().json(db.clone())
}

// 启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(list_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}