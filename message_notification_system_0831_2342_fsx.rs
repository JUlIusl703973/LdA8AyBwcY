use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义消息数据结构
#[derive(Deserialize, Serialize, Debug)]
pub struct Notification {
    pub message: String,
    pub recipient: String,
}

// 定义消息通知服务状态
struct NotificationService {
    notifications: Mutex<HashMap<String, Vec<Notification>>>,
}

// 实现NotificationService
impl NotificationService {
    fn new() -> Self {
        NotificationService {
            notifications: Mutex::new(HashMap::new()),
        }
    }

    fn send_notification(&self, item: Notification) -> Result<HttpResponse, std::io::Error> {
        let mut notifications = self.notifications.lock().unwrap();
        let entry = notifications.entry(item.recipient).or_insert_with(Vec::new);
        entry.push(item);
        Ok(HttpResponse::Ok().json(json!({"status": "message sent"})))
    }
}

// 定义通知处理器
impl Message for Notification {
    type Result = Result<HttpResponse, std::io::Error>;
}

// 创建Http处理器来发送通知
async fn send_notification_handler(service: web::Data<NotificationService>, item: web::Json<Notification>) -> impl Responder {
    service.send_notification(item.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(NotificationService::new()))
            .route("/send", web::post().to(send_notification_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
