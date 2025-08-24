use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post};
use std::fs;
use std::io::{self, Read};
use serde_json::json;

// 函数用于读取文件内容并返回结果
async fn analyze_file_content(path: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    let file_path = path.into_inner();
    
    // 读取文件内容并返回结果
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            // 此处可以添加文件内容分析逻辑
            // 例如：计算单词数量等
            let word_count = content.split_whitespace().count();
            
            // 返回分析结果
            Ok(HttpResponse::Ok().json(json!({
                "file_path": file_path,
                "word_count": word_count
            })))
        },
        Err(_) => {
            // 错误处理
            Err(actix_web::error::ErrorInternalServerError("Failed to read file"))
        },
    }
}

// 主函数
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(analyze_file_content)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
