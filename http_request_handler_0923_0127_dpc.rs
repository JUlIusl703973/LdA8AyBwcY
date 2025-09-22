use actix_web::{get, HttpResponse, Responder};

/// 定义一个结构体，用于处理特定的HTTP请求
struct MyHandler;

/// 定义一个函数来处理GET请求
#[get("/")]
async fn index() -> impl Responder {
    // 这里可以添加更多的逻辑处理
    HttpResponse::Ok().body("Hello World!")
}

/// 定义一个函数来处理GET请求并返回JSON
#[get("/json")]
async fn json_example() -> impl Responder {
    // 这里可以添加更多的逻辑处理
    // 构造一个JSON响应
    let data = r#"{"key": "value"}