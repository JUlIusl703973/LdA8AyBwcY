use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest; // 用于发起网络请求
use select::document::Document; // 用于解析HTML文档
use select::predicate::Name; // 用于CSS选择器
use std::collections::HashMap; // 用于存储数据
use std::error::Error; // 用于错误处理

// 定义一个结构体来存储网页内容
struct WebContent {
    document: Document,
}

impl WebContent {
    // 构造函数
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let html = reqwest::blocking::get(url)?.text()?;
        let doc = Document::from(html.as_str());
        Ok(WebContent { document: doc })
    }

    // 提取网页中的所有链接
    pub fn extract_links(&self) -> Vec<String> {
        self.document
            .find(Name("a")) // 使用CSS选择器选择所有<a>标签
            .filter_map(|n| n.attr("href")) // 获取href属性
            .collect()
    }
}

// 定义一个异步处理函数，用于处理网页抓取请求
async fn handle_scraper(url: web::Path<String>) -> impl Responder {
    match WebContent::new(&url.into_inner()) {
        Ok(content) => {
            let links = content.extract_links();
            HttpResponse::Ok().json(links)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/scraper/{url}", web::get().to(handle_scraper))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
