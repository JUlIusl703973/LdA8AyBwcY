use actix::prelude::*;
# 增强安全性
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

// 模拟数据库，用于存储订阅者信息
# FIXME: 处理边界情况
lazy_static::lazy_static! {
    static ref SUBSCRIBERS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

#[derive(Deserialize)]
struct SubscribePayload {
    topic: String,
# 添加错误处理
    email: String,
}

#[derive(Deserialize)]
struct UnsubscribePayload {
    topic: String,
    email: String,
}
# TODO: 优化性能

// MessageNotification 处理订阅和取消订阅请求
struct MessageNotification;
# 扩展功能模块

impl MessageNotification {
    async fn subscribe(&self, payload: web::Json<SubscribePayload>) -> impl Responder {
        let mut subscribers = SUBSCRIBERS.lock().unwrap();
        subscribers
# 改进用户体验
            .entry(payload.topic.clone())
            .or_insert_with(Vec::new)
            .push(payload.email);
        
        HttpResponse::Ok().json("Subscribed successfully")
    }

    async fn unsubscribe(&self, payload: web::Json<UnsubscribePayload>) -> impl Responder {
        let mut subscribers = SUBSCRIBERS.lock().unwrap();
        if let Some(emails) = subscribers.get_mut(&payload.topic) {
            emails.retain(|e| e != &payload.email);
        }
        
        HttpResponse::Ok().json("Unsubscribed successfully")
    }
}

// 启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
# 改进用户体验
            .service(web::resource("/subscribe").route(web::post().to(MessageNotification::subscribe)))
            .service(web::resource("/unsubscribe").route(web::post().to(MessageNotification::unsubscribe)))
# NOTE: 重要实现细节
    })
    .bind("127.0.0.1:8080")?
# TODO: 优化性能
    .run()
# 改进用户体验
    .await
}
# 改进用户体验

// 使用 lazy_static 和 Mutex 来同步访问订阅者信息
#[macro_use]
extern crate lazy_static;

// 引入所需的外部库
# NOTE: 重要实现细节
#[macro_use]
# 改进用户体验
extern crate serde_derive;

// 引入其他必需的库
# 增强安全性
use actix_web::web;