use actix::prelude::*;
use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use std::time::Instant;
use futures::future::join_all;
use rand::Rng;
use clap::{Arg, Command};

// 定义请求处理函数
async fn index() -> impl Responder {
    "Welcome to the Performance Test!"
}

// 定义性能测试函数
async fn perf_test(url: &str, num: u32) -> impl Responder {
    let mut handles = vec![];
    let start = Instant::now();

    // 创建多个请求处理任务
    for _ in 0..num {
        let url = url.to_string();
        let handle = web::client::Client::new().get(url).send();
        handles.push(handle);
    }

    // 等待所有请求完成
    let responses = join_all(handles).await;

    // 计算总耗时
    let elapsed = start.elapsed();
    let total_time = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

    // 统计成功和失败的请求数量
    let mut success_count = 0;
    let mut error_count = 0;
    for resp in responses {
        if let Ok(response) = resp {
            if response.status().is_success() {
                success_count += 1;
            } else {
                error_count += 1;
            }
        } else {
            error_count += 1;
        }
    }

    format!("Performance Test Results:
Total Time: {:.2?}, Success: {}, Error: {}", total_time, success_count, error_count)
}

// 定义启动服务器的函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 解析命令行参数
    let matches = Command::new("performance_test")
        .arg(Arg::new("url").help("The URL to test").required(true))
        .arg(Arg::new("num").help("The number of requests to send").required(true))
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let num: u32 = matches.value_of("num").unwrap().parse().unwrap();

    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/perf", web::get().to(perf_test))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
