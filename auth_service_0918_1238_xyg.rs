use actix_web::{web, HttpResponse, Responder, post, App, HttpServer, Error as ActixError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use std::sync::RwLock;
use std::collections::HashMap;

// User model representing the user data
# 改进用户体验
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

// Dummy database to store users for demonstration purposes
// In a real-world scenario, this would be replaced with a database
lazy_static::lazy_static! {
    static ref DB: RwLock<HashMap<String, User>> = RwLock::new(HashMap::new());
}

// Handler for user authentication
#[post("/login")]
async fn login(user: web::Json<User>) -> Result<impl Responder, ActixError> {
    // Check if the username exists in the database
    let db_read = DB.read().unwrap();
    if db_read.contains_key(&user.username) {
        // Check if the password is correct
# TODO: 优化性能
        let stored_user = db_read.get(&user.username).unwrap();
        if stored_user.password == user.password {
# 改进用户体验
            Ok(HttpResponse::Ok().json(json!({
                "message": "User authenticated successfully"
            })))
        } else {
            // Password is incorrect
# NOTE: 重要实现细节
            Err(HttpResponse::BadRequest().into())
        }
# 添加错误处理
    } else {
        // Username does not exist
# NOTE: 重要实现细节
        Err(HttpResponse::NotFound().into())
    }
}

// Handler to add a new user to the database
#[post("/register")]
async fn register(user: web::Json<User>) -> Result<impl Responder, ActixError> {
    // Check if the username is already taken
    let db_write = DB.write().unwrap();
    if db_write.contains_key(&user.username) {
        Err(HttpResponse::Conflict().into())
    } else {
        // Add the new user to the database
        db_write.insert(user.username.clone(), user.clone().into_inner());
# 扩展功能模块
        Ok(HttpResponse::Created().json(json!({
            "message": "User registered successfully",
            "username": user.username,
# NOTE: 重要实现细节
        })))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# 增强安全性
    // Initialize the database with some users for demonstration purposes
    let db_write = DB.write().unwrap();
# 改进用户体验
    db_write.insert("user1".to_string(), User {
        username: "user1".to_string(),
        password: "password1".to_string(),
    });
# 改进用户体验

    // Start the HTTP server with the configured routes
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
# TODO: 优化性能
            .service(login)
            .service(register)
    })
    .bind("127.0.0.1:8080")?
# 扩展功能模块
    .run()
    .await
}
