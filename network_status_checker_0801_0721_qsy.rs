use actix_web::{web, App, HttpServer, Responder};
use std::net::SocketAddr;
use std::io;
use tokio::net::TcpStream;
use actix_web::HttpResponse;

// 定义一个结构体来封装网络检查的功能
struct NetworkChecker {
    // 可以添加更多的配置参数，例如超时时间等
    host: String,
    port: u16,
}

// 实现NetworkChecker结构体的方法
impl NetworkChecker {
    // 构造函数
    fn new(host: &str, port: u16) -> Self {
        NetworkChecker {
            host: host.to_string(),
            port,
        }
    }

    // 检查网络连接状态的方法
    async fn check_connection(&self) -> io::Result<HttpResponse> {
        match TcpStream::connect(SocketAddr::new(self.host.parse().unwrap(), self.port)).await {
            Ok(_) => Ok(HttpResponse::Ok().body("Connection successful".to_string())),
            Err(_) => Ok(HttpResponse::InternalServerError().body("Connection failed".to_string())),
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 添加一个路由来处理网络连接状态检查的请求
            .route(