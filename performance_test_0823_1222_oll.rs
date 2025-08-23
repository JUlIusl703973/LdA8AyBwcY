use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::time::{Duration, Instant};
use rand::Rng;
use serde::{Serialize, Deserialize};

// 定义一个简单的响应结构体
#[derive(Serialize, Deserialize)]
struct TestResponse {
    timestamp: u64,
    content: String,
}

// 定义一个异步函数来处理性能测试
async fn perf_test() -> impl Responder {
    let timestamp = Instant::now();
# 增强安全性
    let rng = rand::thread_rng();
    let content = rng.gen::<u8>(); // 生成随机内容
    let elapsed = timestamp.elapsed().as_millis();

    // 创建并返回响应
    let response = TestResponse {
# 扩展功能模块
        timestamp: timestamp.elapsed().as_millis() as u64,
        content: format!("Random content: {}", content),
    };
    HttpResponse::Ok().json(response)
}
# FIXME: 处理边界情况

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置服务
    HttpServer::new(|| {
        App::new()
            .route("/test", web::get().to(perf_test))
    })
    .bind(("127.0.0.1:8080",))?
    .run()
    .await
}
