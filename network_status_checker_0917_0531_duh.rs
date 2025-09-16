use actix::prelude::*;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// 检查给定地址的网络连接状态
async fn check_connection(address: web::Data<String>) -> impl Responder {
    let address = address.into_inner();
    let timeout = Duration::from_secs(5); // 设置超时时间为5秒

    match TcpStream::connect_timeout(&address, timeout).await {
        Ok(_) => HttpResponse::Ok().json("Connection established"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {
                HttpResponse::ServiceUnavailable().json("Connection timed out")
            },
            _ => HttpResponse::BadRequest().json("Failed to establish connection"),
        },
    }
}

/// 程序的主入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(String::from("127.0.0.1:8080"))) // 设置默认地址
            .route("/check", web::post().to(check_connection)) // 定义检查连接状态的路由
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 程序的文档字符串
/// 网络连接状态检查器
///
/// 这个程序提供了一个简单的网络连接状态检查功能，可以通过发送POST请求到"/check"端点来检查指定的网络地址是否可达。
///
/// # 示例
/// 使用curl发送POST请求：
/// ```
/// curl -d "127.0.0.1:8080" -X POST http://localhost:8080/check
/// ```
