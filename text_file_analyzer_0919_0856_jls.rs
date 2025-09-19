use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io::{self, Read};
use std::path::Path;

/// 分析文本文件内容，并返回分析结果
async fn analyze_text_file(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();
    let analysis_result = match analyze_file(&file_path) {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    HttpResponse::Ok().json(analysis_result)
}

/// 读取文件内容并进行分析
fn analyze_file(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 示例分析：计算文件中的单词数量
    let word_count = contents.split_whitespace().count();

    Ok(format!("文件'{}'中的单词数量为: {}", path, word_count))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .route("/analyze", web::post().to(analyze_text_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// 定义模块和函数的文档
///
/// 文件内容分析器
///
/// 该模块定义了一个文本文件内容分析器，能够计算并返回指定文件中的单词数量。
///
/// 示例用法：
/// 向服务器发送POST请求到/analyze，请求体中包含文件路径。
///
/// 请注意，实际分析逻辑可以根据需求进行扩展和自定义。
///
/// # 错误处理
///
/// 如果文件读取或分析过程中出现错误，将返回HTTP 500错误响应。
