use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use once_cell::sync::Lazy;

// 配置文件路径
static CONFIG_PATH: Lazy<String> = Lazy::new(|| "strings::to_string("config.toml")");

#[derive(Debug)]
struct FileSyncConfig {
    src: String,
    dst: String,
}

/// 读取配置文件
fn read_config() -> io::Result<FileSyncConfig> {
    let config = fs::read_to_string(CONFIG_PATH.as_str())?;
    let config: FileSyncConfig = toml::from_str(&config)?;
    Ok(config)
}

/// 同步文件内容
fn sync_files(src: &str, dst: &str) -> io::Result<()> {
    let source = Path::new(src);
    let destination = Path::new(dst);

    if !source.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source file not found"));
    }

    if source.is_dir() {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(source)?.to_str().unwrap();
            let target_path = destination.join(relative_path);
            sync_files(path.to_str().unwrap(), target_path.to_str().unwrap())?;
        }
    } else {
        fs::copy(source, destination)?;
    }
    Ok(
        ()
    )
}

/// Actix Web 服务端点
async fn sync_endpoint() -> impl Responder {
    let config = match read_config() {
        Ok(config) => config,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match sync_files(&config.src, &config.dst) {
        Ok(_) => HttpResponse::Ok().body("Files synchronized successfully."),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sync", web::get().to(sync_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
