use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use serde::Serialize;
use serde_json::json;

// 定义一个结构体来存储文件分析的结果
#[derive(Serialize)]
struct FileAnalysisResult {
    total_lines: usize,
    total_characters: usize,
    total_words: usize,
}

#[get("/analyze/{path}")]
async fn analyze_file(path: web::Path<String>) -> impl Responder {
    // 从路径参数中提取文件路径
    let file_path = path.into_inner();
    let path = Path::new(&file_path);

    // 检查文件是否存在
    if !path.exists() {
        return HttpResponse::NotFound().json(json!({"error": "File not found"}));
    }

    // 打开文件并创建一个BufReader来逐行读取
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Failed to open file"})),
    };
    let reader = BufReader::new(file);

    // 初始化计数器
    let mut total_lines = 0;
    let mut total_characters = 0;
    let mut total_words = 0;

    // 逐行读取文件并更新计数器
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => continue, // 在这里选择忽略错误
        };
        total_lines += 1;
        total_characters += line.len();
        total_words += line.split_whitespace().count();
    }

    // 将结果序列化为JSON并返回
    let result = FileAnalysisResult {
        total_lines,
        total_characters,
        total_words,
    };
    HttpResponse::Ok().json(json!(result))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(analyze_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
