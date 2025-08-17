// text_file_analyzer.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io::Read;
use std::path::Path;
use actix_web::error::ErrorBadRequest;
use actix_web::error::ErrorInternalServerError;

// 定义一个结构体，用于处理文件内容分析的逻辑
struct TextFileAnalyzer;

impl TextFileAnalyzer {
    // 分析文本文件并返回分析结果
    pub fn analyze_file(path: &Path) -> Result<String, std::io::Error> {
        // 读取文件内容
        let mut content = String::new();
        let mut file = fs::File::open(path)?;
        file.read_to_string(&mut content)?;

        // 这里可以添加更多的分析逻辑，例如统计字数、行数等
        let analysis_result = format!("Analysis of file '{}': contains {} characters", path.display(), content.chars().count());

        Ok(analysis_result)
    }
}

// 定义一个处理函数，用于处理分析请求
async fn analyze_text_file(path: web::Path<String>) -> Result<impl Responder, ErrorBadRequest> {
    let analysis_result = TextFileAnalyzer::analyze_file(Path::new(&path)).map_err(|_| ErrorBadRequest("Failed to analyze file."))?;
    
    // 返回分析结果
    HttpResponse::Ok().json(analysis_result)
}

// 启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 路由处理分析文件的请求
            .route("/analyze", web::post().to(analyze_text_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
