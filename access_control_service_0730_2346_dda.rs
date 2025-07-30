use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Result};
# TODO: 优化性能
use actix_web::middleware::{DefaultHeaders, Logger};
# 增强安全性
use futures::future::{ok, Ready};
# 优化算法效率
use std::sync::Arc;

// 定义一个简单的用户结构体
struct User {
    username: String,
# 扩展功能模块
    is_admin: bool,
}
# FIXME: 处理边界情况

// 模拟的用户数据
const USERS: [User; 2] = [
    User { username: "alice".to_string(), is_admin: false },
    User { username: "bob".to_string(), is_admin: true },
];

// 创建一个简单的权限检查函数
fn check_admin(user: &str) -> Ready<Result<bool, String>> {
    ok(USERS.iter().any(|u| u.username == user && u.is_admin))
# 扩展功能模块
}
# 增强安全性

// 定义一个带有权限控制的路由
# NOTE: 重要实现细节
#[get("/admin")]
async fn admin_route(user: web::Path<String>) -> impl Responder {
    match check_admin(&user.into_inner()).await {
        Ok(true) => HttpResponse::Ok().body("Welcome to the admin panel!"),
        Ok(false) => HttpResponse::Forbidden().body("Access denied!"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error!"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add("Server", "Actix-web"))
# TODO: 优化性能
            .service(admin_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
