use actix_web::{get, HttpResponse, Responder, web};
use std::process::Command;
use std::str;
use serde::Serialize;
use serde_json::json;

// 定义内存使用情况结构体
#[derive(Serialize)]
struct MemoryUsage {
    rss: u64,
    vsize: u64,
    swap: u64,
}

// 实现内存使用情况的函数
async fn get_memory_usage() -> Result<MemoryUsage, &'static str> {
    // 执行 `ps` 命令获取内存使用信息
    let output = Command::new("ps")
        .args("-p")
        .arg("1")
        .arg("-o")
        .arg("rss,vsize,swap")
        .output()
        .map_err(|_| "Failed to execute ps command")?;

    // 将输出转换为字符串
    let output_str = str::from_utf8(&output.stdout).map_err(|_| "Failed to parse output")?;

    // 解析内存使用信息
    let mut parts = output_str.split_whitespace();
    let rss = parts
        .next()
        .and_then(|part| part.parse::<u64>().ok())
        .ok_or("Failed to parse rss")?;
    let vsize = parts
        .next()
        .and_then(|part| part.parse::<u64>().ok())
        .ok_or("Failed to parse vsize")?;
    let swap = parts
        .next()
        .and_then(|part| part.parse::<u64>().ok())
        .ok_or("Failed to parse swap")?;

    // 返回内存使用情况
    Ok(MemoryUsage { rss, vsize, swap })
}

// 实现Actix Web的HTTP接口
#[get("/memory")]
async fn memory_endpoint() -> impl Responder {
    match get_memory_usage().await {
        Ok(memory_usage) => HttpResponse::Ok().json(json!(memory_usage)),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置Logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 启动Actix Web服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(memory_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
