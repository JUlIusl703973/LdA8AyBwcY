use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::net::TcpStream;
use std::io::Result as IoResult;

/// 定义一个检查网络连接状态的结构体
struct NetworkStatusChecker;

/// 使用actix_web的get宏来定义一个路由
/// `/check_status` 路由将返回网络连接状态
#[get("/check_status")]
async fn check_status() -> impl Responder {
    let result = check_network_connection("github.com:443").await;
    match result {
        Ok(is_connected) => {
            if is_connected {
                HttpResponse::Ok().body("Network is connected.")
            } else {
                HttpResponse::InternalServerError().body("Network is not connected.")
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// 异步函数，用于检查到指定服务器的TCP连接
async fn check_network_connection(address: &str) -> IoResult<bool> {
    let conn = TcpStream::connect(address).await;
    match conn {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// 启动服务的main函数
#[actix_web::main]
async fn main() -> IoResult<()> {
    // 定义http服务监听的地址和端口
    let server = HttpServer::new(|| {
        App::new()
            .service(check_status)
    })
    .bind(("127.0.0.1:8080",))?
    .run();

    println!("Server running at http://127.0.0.1:8080");
    server.await
}
