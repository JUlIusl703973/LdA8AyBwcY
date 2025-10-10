use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
# FIXME: 处理边界情况
use std::io;
use std::path::Path;
use std::fs;

// 定义文档转换请求的数据结构
#[derive(Serialize, Deserialize)]
struct ConvertRequest {
    input_file: String,
    output_format: String,
}

// 定义文档转换响应的数据结构
#[derive(Serialize, Deserialize)]
struct ConvertResponse {
    status: String,
    message: String,
}

#[post("/convert")]
async fn convert_document(req_body: web::Json<ConvertRequest>) -> impl Responder {
    let input_file = req_body.input_file.clone();
    let output_format = req_body.output_format.clone();

    // 检查文件是否存在
    if !Path::new(&input_file).exists() {
        return HttpResponse::BadRequest().json(ConvertResponse {
            status: "error".to_string(),
            message: "Input file does not exist".to_string(),
        });
    }

    // 这里只是一个示例，实际的转换逻辑需要根据文件类型和输出格式来实现
    // 假设我们只是简单地复制文件，不进行真正的格式转换
    let output_file = format!("{}_converted.{}", input_file, output_format);
    match fs::copy(input_file, output_file) {
        Ok(_) => HttpResponse::Ok().json(ConvertResponse {
            status: "success".to_string(),
            message: "Document converted successfully".to_string(),
        }),
# 优化算法效率
        Err(e) => HttpResponse::InternalServerError().json(ConvertResponse {
            status: "error".to_string(),
# 改进用户体验
            message: format!("Failed to convert document: {}", e),
        }),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
# FIXME: 处理边界情况
    HttpServer::new(|| {
        App::new()
            .service(convert_document)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
