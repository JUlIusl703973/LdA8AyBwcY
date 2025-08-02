// web_content_scraper.rs
//
// 网页内容抓取工具，使用 Rust 语言和 Actix 框架实现。
//

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use std::error::Error;
use serde_json::json;

// 定义一个结构体，用于存储网页内容抓取工具的状态
struct WebContentScraper;

impl WebContentScraper {
    // 异步函数，用于抓取网页内容
    async fn fetch_content(&self, url: String) -> Result<String, Box<dyn Error>> {
        // 使用 Reqwest 库发起 HTTP GET 请求
        let response = reqwest::get(&url).await?;
        
        // 检查响应状态码
        if response.status().is_success() {
            // 读取响应体为字符串
            Ok(response.text().await?)
        } else {
            // 如果响应状态码不是成功状态，返回错误
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch content")))
        }
    }
}

// 定义路由处理函数
async fn fetch_web_content(url: web::Path<String>) -> impl Responder {
    let scraper = WebContentScraper;
    match scraper.fetch_content(url.into_inner()).await {
        Ok(content) => HttpResponse::Ok().json(json!{"content": content}),
        Err(e) => HttpResponse::InternalServerError().json(json!{"error": e.to_string()}),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置服务器监听的地址和端口
    HttpServer::new(|| {
        App::new()
            // 注册路由，将 URL 映射到处理函数
            .route("/fetch", web::get().to(fetch_web_content))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
