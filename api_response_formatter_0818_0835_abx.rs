use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;
use serde_json::json;

// 定义一个结构体来表示API响应
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: T,
}

// 实现一个API响应格式化工具
#[get("/format")]
async fn format_response() -> impl Responder {
    // 模拟一些数据
    let data = "Hello, Actix!";
    
    // 创建一个成功的响应
    let response = ApiResponse {
        success: true,
        message: "Request successful".to_string(),
        data,
    };

    // 序列化响应为JSON
    let json_response = json!(response);
    
    // 返回一个成功的HTTP响应
    HttpResponse::Ok().json(json_response)
}

// 定义Actix应用
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 添加路由
            .route("/format", get(format_response))
    })
    // 监听端口
    .bind("127.0.0.1:8080")?
    .run()
    .await
}