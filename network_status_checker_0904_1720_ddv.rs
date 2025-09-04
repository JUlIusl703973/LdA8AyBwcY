use actix_web::{get, HttpResponse, Responder, web};
use std::net::TcpStream;
use std::io::ErrorKind;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use async_trait::async_trait;

/// NetworkStatusChecker 结构体，用于检查网络连接状态
struct NetworkStatusChecker;

/// 检查给定服务器的连接状态
#[async_trait]
trait CheckConnection {
    async fn check(&self, server: &str, timeout_ms: u64) -> bool;
}

#[async_trait]
impl CheckConnection for NetworkStatusChecker {
    /// 实现检查连接状态的方法
    async fn check(&self, server: &str, timeout_ms: u64) -> bool {
        // 使用超时时间来尝试连接服务器
        match timeout(Duration::from_millis(timeout_ms), TcpStream::connect(server)).await {
            Ok(result) => match result {
                Ok(_) => true, // 连接成功
                Err(e) => {
                    if e.kind() == ErrorKind::TimedOut {
                        false // 超时，连接失败
                    } else {
                        false // 其他错误，连接失败
                    }
                },
            },
            Err(_) => false, // 超时错误，连接失败
        }
    }
}

/// 定义路由处理函数
#[get("/check/{server}")]
async fn check_server_status(server: web::Path<String>, web::Query(timeout): web::Query<u64>) -> impl Responder {
    let checker = NetworkStatusChecker;
    // 检查服务器连接状态
    let is_connected = checker.check(&server.into_inner(), timeout).await;
    // 返回HTTP响应
    match is_connected {
        true => HttpResponse::Ok().json({
            "status": "Connected"
        }),
        false => HttpResponse::InternalServerError().json({
            "status": "Not Connected"
        }),
    }
}

/// main函数，启动Actix服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置和启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(check_server_status)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
