use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

// 定义一个全局的消息存储，使用Arc和Mutex来实现线程安全
lazy_static! {
    static ref MESSAGES: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}
# 扩展功能模块

// 定义消息通知请求结构体
#[derive(Serialize, Deserialize)]
struct NotificationRequest {
    message: String,
    recipient: String,
}

#[get("/messages/{recipient}")]
async fn get_messages(recipient: web::Path<String>) -> impl Responder {
    let messages = MESSAGES.lock().unwrap();
    let recipient_messages = messages.get(&recipient.into_inner()).unwrap_or(&String::new()).clone();
    HttpResponse::Ok().json(json!({
        "recipient": recipient.into_inner(),
        "messages": vec![recipient_messages],
# 增强安全性
    }))
}

#[post("/send")]
async fn send_notification(req_body: web::Json<NotificationRequest>) -> impl Responder {
    let mut messages = MESSAGES.lock().unwrap();
# 改进用户体验
    let recipient = req_body.recipient.clone();
# 扩展功能模块
    if let Some(existing_messages) = messages.get_mut(&recipient) {
# NOTE: 重要实现细节
        *existing_messages = format!("{}
{}", existing_messages, req_body.message);
    } else {
        messages.insert(recipient, req_body.message);
    }
    HttpResponse::Ok().json(json!({
        "message": "Notification sent successfully",
# NOTE: 重要实现细节
        "details": {
            "recipient": recipient,
            "message": req_body.message,
        },
    }))
# 改进用户体验
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_messages)
            .service(send_notification)
    })
# 添加错误处理
    .bind("127.0.0.1:8080")?
# 扩展功能模块
    .run()
    .await
}
# FIXME: 处理边界情况

// 使用lazy_static宏来实现全局消息存储的线程安全
#[macro_use]
extern crate lazy_static;

// 添加必要的依赖项
// [dependencies]
// actix-web = "4"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// lazy_static = "1.4"
# 增强安全性