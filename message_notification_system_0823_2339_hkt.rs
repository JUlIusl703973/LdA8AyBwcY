use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as _};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// 定义一个简单的用户结构体
#[derive(Clone)]
struct User {
    id: u32,
    name: String,
}

// 定义消息结构体，包含消息内容和消息类型
#[derive(Clone)]
struct Message {
    id: u32,
    content: String,
    message_type: String,
}

// 创建一个消息通知服务
struct NotificationService {
    users: Arc<Mutex<HashMap<u32, User>>>,
    messages: Arc<Mutex<Vec<Message>>>,
}

// 定义接收消息的请求体结构
#[derive(Deserialize)]
struct ReceiveMessageRequest {
    user_id: u32,
}

// 实现NotificationService
impl NotificationService {
    // 添加新用户
    fn add_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user);
    }

    // 发送消息
    fn send_message(&self, message: Message) {
        let mut messages = self.messages.lock().unwrap();
        messages.push(message);
    }

    // 接收消息
    fn receive_message(&self, req: ReceiveMessageRequest) -> impl Responder {
        let user_id = req.user_id;
        let mut found = false;
        let mut message_content = String::new();

        let messages = self.messages.lock().unwrap();
        for message in messages.iter() {
            if message.message_type == 