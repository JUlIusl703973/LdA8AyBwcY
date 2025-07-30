use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// 定义一个请求结构体，用于解析传入的数据文件路径
#[derive(Deserialize)]
pub struct AnalyzeRequest {
    file_path: String,
}

// 数据统计分析器，用于读取文件并进行统计分析
struct DataAnalyzer;

impl DataAnalyzer {
    // 构造函数
    pub fn new() -> Self {
        DataAnalyzer
    }

    // 读取文件并执行统计分析
    pub fn analyze(&self, file_path: &str) -> io::Result<Vec<String>> {
        let path = Path::new(file_path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut results = Vec::new();
        for line in reader.lines() {
            let line = line?;
            // 在这里添加具体的统计分析逻辑
            // 例如，计算行数，单词数等
            results.push(line);
        }

        Ok(results)
    }
}

#[get("/analyze")]
async fn analyze_data(req: HttpRequest) -> impl Responder {
    let data_analyzer = DataAnalyzer::new();
    let file_path = req.match_info().get("file_path").unwrap_or("");
    match data_analyzer.analyze(file_path) {
        Ok(results) => HttpResponse::Ok().json(json!{{"results": results}}),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(analyze_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_analyze_data() {
        let req = test::TestRequest::with_uri("/analyze/some_file.txt").to_request();
        let resp = analyze_data(req).await;
        assert!(resp.status().is_success());
    }
}
