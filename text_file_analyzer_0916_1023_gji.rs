use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

/// 这个结构体负责文本文件内容分析
pub struct TextFileAnalyzer;

impl TextFileAnalyzer {
    /// 分析文本文件内容
    pub async fn analyze_content(file_path: String) -> impl Responder {
        let path = Path::new(&file_path);
        if !path.exists() {
            return HttpResponse::NotFound().body("File not found");
        }
        
        let content = fs::read_to_string(path).map_err(|_| "Failed to read file");
        match content {
            Ok(text) => {
                let analysis = TextFileAnalyzer::perform_analysis(&text);
                HttpResponse::Ok().json(analysis)
            }
            Err(_) => HttpResponse::InternalServerError().body("Error reading file"),
        }
    }

    /// 执行文本分析（这里只是一个示例，可以根据需要实现具体的分析功能）
    fn perform_analysis(text: &str) -> AnalysisResult {
        AnalysisResult {
            line_count: text.lines().count(),
            word_count: text.split_whitespace().count(),
        }
    }
}

/// 分析结果
pub struct AnalysisResult {
    pub line_count: usize,
    pub word_count: usize,
}

impl actix_web::ResponseError for std::io::Error {}

/// 设置HTTP服务器和路由
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/analyze", web::post().to(TextFileAnalyzer::analyze_content))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
