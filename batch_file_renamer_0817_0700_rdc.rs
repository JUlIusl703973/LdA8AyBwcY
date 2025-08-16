// batch_file_renamer.rs

// 引入必要的库
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs;
use std::path::Path;
use regex::Regex;

// 定义一个结构体来存储配置信息
struct RenameConfig {
    pattern: Regex,
    format: String,
}

// 实现 RenameConfig 的方法
impl RenameConfig {
    // 构造函数
    pub fn new(pattern: &str, format: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: Regex::new(pattern)?,
            format: format.to_string(),
        })
    }

    // 重命名文件的方法
    pub fn rename_files(&self, dir: &str) -> Result<(), std::io::Error> {
        // 列出目录下所有文件
        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // 匹配文件名
                if let Some(caps) = self.pattern.captures(&path.file_name().unwrap().to_string_lossy()) {
                    let new_name = self.format.replace("{}", &caps[1]);
                    let new_path = path.with_file_name(new_name);
                    fs::rename(&path, &new_path)?;
                }
            }
        }
        Ok(())
    }
}

// 定义一个结构体用于处理 HTTP 请求
struct RenameHandler {
    config: RenameConfig,
}

// 实现 RenameHandler 的方法
impl RenameHandler {
    // 构造函数
    pub fn new(config: RenameConfig) -> Self {
        RenameHandler { config }
    }

    // HTTP POST 请求处理
    async fn rename(&self) -> impl Responder {
        let dir = "/path/to/directory"; // 指定目录
        match self.config.rename_files(dir) {
            Ok(_) => HttpResponse::Ok().json("Files renamed successfully"),
            Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建一个 RenameConfig 实例
    let config = RenameConfig::new(r"^old_(\d+)\.txt$", "new_{}\.txt").unwrap();

    // 创建一个 RenameHandler 实例
    let handler = RenameHandler::new(config);

    // 设置 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/rename").route(web::post().to(
                handler.rename
            )))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
