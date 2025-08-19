use actix_web::{get, HttpResponse, Responder, web};
use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;

// 使用lazy_static来定义全局的错误日志存储
lazy_static! {
    static ref ERROR_LOGS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

// 定义一个结构体，用于管理错误日志
struct ErrorLogger;

#[get("/log_error/{name}")]
async fn log_error(name: web::Path<String>) -> impl Responder {
    let mut logs = ERROR_LOGS.lock().unwrap();
    logs.entry(name.into_inner()).or_insert_with(Vec::new);
    logs.get_mut(&name.into_inner()).map(|v| v.push(format!("Error logged at {}", chrono::Local::now().to_rfc3339())));

    HttpResponse::Ok().body("Error logged successfully")
}

#[get("/get_logs")]
async fn get_logs() -> impl Responder {
    let logs = ERROR_LOGS.lock().unwrap();
    HttpResponse::Ok().json(logs)
}

// 应用配置
fn main() -> std::io::Result<()> {
    // 初始化Actix Web应用
    let app = actix_web::web::App::new()
        .service(web::resource("/log_error/{name}").to(log_error))
        .service(web::resource("/get_logs").to(get_logs));

    // 运行服务器
    actix_web::HttpServer::new(|| app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// 引入必要的模块
#[macro_use]
extern crate lazy_static;
use chrono::Local;
