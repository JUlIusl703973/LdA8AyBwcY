use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use std::sync::Mutex;
use lazy_static::lazy_static;

// 定义一个全局的Mutex来安全地在多线程环境中共享错误日志
lazy_static! {
    static ref ERROR_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// 用于记录错误的函数
fn log_error(error_message: String) {
    ERROR_LOG.lock().unwrap().push(error_message);
}

/// 获取错误日志的端点
#[get("/error_log")]
async fn get_error_log() -> impl Responder {
    let errors = ERROR_LOG.lock().unwrap();
    HttpResponse::Ok().json(errors.clone())
}

/// 模拟发生错误的端点
#[get("/trigger_error")]
async fn trigger_error() -> impl Responder {
    let error_message = "Something went wrong!".to_string();
    log_error(error_message.clone());
    HttpResponse::InternalServerError().json("Error triggered")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_error_log)
            .service(trigger_error)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 使用`lazy_static`宏来创建全局静态变量
#[macro_use] extern crate lazy_static;
extern crate actix_web;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;
