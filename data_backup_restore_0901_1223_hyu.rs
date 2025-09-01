// data_backup_restore.rs
//
// 该程序使用RUST语言和ACTIX框架实现数据备份和恢复功能。

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json::json;

// 定义备份文件的路径
const BACKUP_PATH: &str = "./backups/";

#[get("/backup")]
// 创建GET路由，用于触发数据备份操作
async fn backup_data() -> impl Responder {
    let backup_path = Path::new(BACKUP_PATH);
    if !backup_path.exists() {
        fs::create_dir_all(backup_path).unwrap_or_else(|_| panic!("Failed to create backup directory"));
    }

    let mut backup_file = File::create(backup_path.join("backup.dat")).unwrap_or_else(|_| panic!("Failed to create backup file"));
    let data = "Your data here"; // 这里应替换为实际需要备份的数据
    backup_file.write_all(data.as_bytes()).unwrap_or_else(|_| panic!("Failed to write to backup file"));

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Data backed up successfully"
    }))
}

#[post("/restore")]
// 创建POST路由，用于触发数据恢复操作
async fn restore_data() -> impl Responder {
    let backup_file_path = Path::new(BACKUP_PATH).join("backup.dat");
    if !backup_file_path.exists() {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Backup file not found"
        }))
    }

    let mut file = File::open(backup_file_path).unwrap_or_else(|_| panic!("Failed to open backup file"));
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap_or_else(|_| panic!("Failed to read backup file"));

    // 这里应替换为实际需要恢复数据的操作
    // 例如，将备份数据写入数据库或文件系统

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Data restored successfully"
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .service(backup_data)
            .service(restore_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
