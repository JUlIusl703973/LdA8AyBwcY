use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::header::CONTENT_DISPOSITION;
use std::io::Cursor;
use serde::Serialize;
use serde_json::json;

// 定义一个结构体来存储表格数据
#[derive(Serialize)]
pub struct SheetData {
    pub rows: Vec<Vec<String>>,
}

// Excel文件的MIME类型
const MIME_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

#[get("/generate_excel")]
// 实现一个HTTP GET请求处理器，生成Excel文件
async fn generate_excel() -> impl Responder {
    let data = SheetData {
        rows: vec![
            vec!["Header1".to_string(), "Header2".to_string(), "Header3".to_string()],
            vec!["Row1Col1".to_string(), "Row1Col2".to_string(), "Row1Col3".to_string()],
            vec!["Row2Col1".to_string(), "Row2Col2".to_string(), "Row2Col3".to_string()],
        ],
    };

    // 将数据转换为JSON格式的字符串
    let json_data = serde_json::to_string(&data).expect("Failed to serialize data to JSON");

    // 创建一个Cursor，用于存储Excel文件的内容
    let mut cursor = Cursor::new(Vec::new());

    // 使用rust-xlsxwriter库将JSON数据写入Excel文件
    let mut writer = xlsxwriter::XlsxWriter::new(cursor);
    if let Err(e) = writer.write_json(&json_data) {
        // 如果写入Excel文件失败，返回错误信息
        eprintln!("Failed to write Excel file: {}", e);
        return HttpResponse::InternalServerError().json(json!({"error": "Failed to write Excel file"}));
    }
    writer.close().expect("Failed to close writer");

    // 设置Content-Disposition头部，以便浏览器可以下载文件
    let content_disposition = format!("attachment; filename=example.xlsx");
    HttpResponse::Ok()
        .insert_header((CONTENT_DISPOSITION, content_disposition))
        .content_type(MIME_TYPE)
        .body(cursor.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动Actix服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(generate_excel)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
