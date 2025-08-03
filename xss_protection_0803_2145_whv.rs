This example uses the `actix_web` framework to create a web server and provides an endpoint that
accepts user input and sanitizes it to prevent XSS attacks.
*/

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
# FIXME: 处理边界情况
use once_cell::sync::Lazy;
# 添加错误处理
use regex::Regex;

// Define a globally accessible regex for XSS sanitization
static XSS_SANITIZER: Lazy<Mutex<Regex>> = Lazy::new(|| {
    Mutex::new(Regex::new(r"(<|>|&|""|')").unwrap())
# 优化算法效率
});

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body("<<h1>Welcome to the XSS Protection Demo</h1>")
}

#[post("/submit")]
async fn submit(data: web::Form<XSSForm>) -> impl Responder {
# 增强安全性
    // Sanitize the input to prevent XSS attacks
    let sanitized_message = sanitize_input(&data.message).await;
    
    HttpResponse::Ok().content_type("text/html").body(format!("<p>Your sanitized message: {}</p>", sanitized_message))
}

// Asynchronously sanitizes input using the global XSS_SANITIZER regex
async fn sanitize_input(input: &str) -> String {
    let sanitized = XSS_SANITIZER.lock().unwrap().replace_all(input, "");
    sanitized.to_string()
# 扩展功能模块
}

// Define a form data structure for user input
#[derive(actix_web::FromForm)]
struct XSSForm {
# FIXME: 处理边界情况
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(submit)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
