use actix_web::{web, App, HttpServer, Responder, get};
use serde::Deserialize;
use serde_json::json;

// 定义一个结构体来处理数据清洗请求
#[derive(Deserialize)]
struct CleanDataRequest {
    // 这里可以定义需要的数据字段
    input_data: String,
}

// 定义响应结构体
#[derive(Serialize)]
struct CleanDataResponse {
    cleaned_data: String,
}

// 实现数据清洗功能
fn clean_data(data: &str) -> String {
    // 这里添加数据清洗和预处理的逻辑
    // 示例：去除字符串中的空格
    data.trim().to_string()
}

// 创建一个HTTP GET端点来处理数据清洗请求
#[get("/clean")]
async fn handle_clean_data(req_data: web::Json<CleanDataRequest>) -> impl Responder {
    let cleaned = clean_data(&req_data.input_data);
    let response = CleanDataResponse { cleaned_data: cleaned };
    web::Json(json!(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(handle_clean_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}