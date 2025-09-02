use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// 定义一个简单的数据结构用于演示
#[derive(Debug, Clone)]
struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// 实现一个handler来处理GET请求，模拟查询用户信息
async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    // 模拟数据库查询
    let users = vec![
        User { id: 1, name: "John Doe".to_string(), email: "john.doe@example.com".to_string() },
        User { id: 2, name: "Jane Doe".to_string(), email: "jane.doe@example.com".to_string() },
    ];

    let user = users.iter()
        .find(|u| u.id == user_id.into_inner())
        .cloned();

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

// 定义启动服务器的main函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置监听地址和端口
    HttpServer::new(|| {
        App::new()
            .route("/user/{}