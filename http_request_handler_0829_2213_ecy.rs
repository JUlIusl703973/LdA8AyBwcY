use actix_web::{web, App, HttpServer, Responder};
use actix_web::HttpResponse;
use std::io;

// 定义一个简单的结构体，用于处理请求
struct HttpRequestHandler;

// 实现HttpRequestHandler的trait，使其能够作为HTTP请求处理器使用
impl HttpRequestHandler {
    // 定义一个处理GET请求的方法
    async fn get_handler() -> impl Responder {
        // 返回一个HTTP响应，状态码为200，内容为"Hello, World!"
        HttpResponse::Ok().body("Hello, World!")
    }

    // 定义一个处理POST请求的方法
    async fn post_handler() -> impl Responder {
        // 这里可以添加处理POST请求的逻辑，例如从请求体中读取数据
        // 为了示例简单，这里直接返回一个HTTP响应
        HttpResponse::Ok().body("Received POST request")
    }
}

// 定义main函数，启动HTTP服务器
#[actix_web::main]
async fn main() -> io::Result<()> {
    // 创建一个HttpServer，并指定监听地址和端口
    HttpServer::new(|| {
        // 使用App，配置路由和中间件
        App::new()
            // 配置GET请求的路由，路径为"/"，处理器为HttpRequestHandler的get_handler方法
            .route("/", web::get().to(HttpRequestHandler::get_handler))
            // 配置POST请求的路由，路径为"/post"，处理器为HttpRequestHandler的post_handler方法
            .route("/post", web::post().to(HttpRequestHandler::post_handler))
    })
    // 指定服务器监听的地址和端口（例如：localhost:8080）
    .bind("127.0.0.1:8080")?
    // 启动服务器
    .run()
    // 等待服务器运行结果
    .await
}
