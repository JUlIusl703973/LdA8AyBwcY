// backup_restore_service.rs
//
// 实现数据备份和恢复的服务。
//
// 遵循RUST最佳实践，确保代码的可维护性和可扩展性。

use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::json;

// 定义备份文件的结构体
#[derive(Serialize, Deserialize, Debug)]
struct BackupFile {
    filename: String,
    content: String,
}

// 实现备份功能
#[get("/backup")]
async fn backup() -> impl Responder {
    let data_to_backup = "Data to backup";
    let backup_filename = "backup_data.json";
    let backup_path = Path::new(backup_filename);

    match fs::write(backup_path, data_to_backup) {
        Ok(_) => HttpResponse::Ok().json(json!{"message": "Backup successful", "filename": backup_filename}),
        Err(e) => HttpResponse::InternalServerError().json(json!{"error": format!("Failed to backup: {}", e)}),
    }
}

// 实现恢复功能
#[get("/restore")]
async fn restore() -> impl Responder {
    let backup_filename = "backup_data.json";
    let backup_path = Path::new(backup_filename);

    match fs::read_to_string(backup_path) {
        Ok(content) => HttpResponse::Ok().json(json!{"message": "Restore successful", "data": content}),
        Err(e) => HttpResponse::InternalServerError().json(json!{"error": format!("Failed to restore: {}", e)}),
    }
}

// 启动服务器
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(backup)
            .service(restore)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
