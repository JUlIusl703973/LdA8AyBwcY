// text_file_analyzer.rs
// 这是一个基于Rust和Actix框架的简单文本文件内容分析器。
// 它读取文本文件，分析其内容，并返回一些基本的统计信息。

use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, error};
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// 分析文本文件并返回统计信息
/// 此函数打开指定的文件，读取内容，并计算字数
async fn analyze_text_file(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();
    let mut word_count = 0;
    let mut line_count = 0;
    let mut char_count = 0;

    // 打开文件并读取内容
    let file = match fs::File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };
        word_count += line.split_whitespace().count();
        line_count += 1;
        char_count += line.len();
    }

    HttpResponse::Ok().json(web::Json(AnalysisResult {
        word_count,
        line_count,
        char_count,
    }))
}

/// 定义返回的统计信息结构体
#[derive(serde::Serialize)]
struct AnalysisResult {
    word_count: usize,
    line_count: usize,
    char_count: usize,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/analyze", web::post().to(analyze_text_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
