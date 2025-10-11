use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, Error, Result, Responder as ActixResponder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// 模拟数据库存储，使用HashMap和Mutex来模拟锁
lazy_static::lazy_static! {
    static ref DB: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 用户信息结构体
#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    username: String,
    password: String,
}

// 用户注册请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct RegisterRequest {
    username: String,
    password: String,
}

// 隐私保护处理器
#[post("/register")]
async fn register_user(info: web::Json<RegisterRequest>) -> ActixResponder {
    // 检查用户名是否已存在
    let mut db = DB.lock().unwrap();
    if db.contains_key(&info.username) {
        return HttpResponse::Conflict().json(json!({"error": "Username already exists"}));
    }
    
    // 模拟加密密码（在实际应用中应使用安全的加密算法）
    let encrypted_password = encrypt_password(&info.password);
    
    // 将用户信息存储到模拟数据库
    db.insert(info.username.clone(), encrypted_password);
    
    HttpResponse::Ok().json(json!({"message": "User registered successfully"}))
}

// 注册路由
#[get("/privacy")]
async fn privacy_policy() -> impl Responder {
    "Privacy policy: This application respects user privacy and only stores encrypted data."
}

// 加密密码的函数（示例，实际应用中应使用更安全的加密方法）
fn encrypt_password(password: &str) -> String {
    format!("encrypted_{}", password)
}

// 启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register_user)
            .service(privacy_policy)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 使用lazy_static宏定义静态变量
#[macro_use]
extern crate lazy_static;

// 引入必要的crate
#[macro_use]
extern crate serde_derive;

// 引入模块
use actix_web::web;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;

// 依赖项
fn main() {}
