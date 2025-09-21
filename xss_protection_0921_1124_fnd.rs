use actix_web::{get, post, HttpResponse, Responder, web, App, HttpServer, HttpRequest};
use actix_web::http::StatusCode;
use regex::Regex;

/// 这个函数用于检测输入文本是否可能包含XSS攻击代码
/// 它使用正则表达式来匹配可能的XSS模式
fn detect_xss(text: &str) -> bool {
    let regex = Regex::new(r"(<|>|"|')").unwrap();
    regex.is_match(text)
}

/// 这个函数用于清理输入文本，以防止XSS攻击
/// 它将所有可能的HTML标签替换为空字符串
fn sanitize_input(text: &str) -> String {
    text.replace("<", "").replace(">", "").replace(""", "").replace("'", "")
}

/// 这是一个处理GET请求的路由
/// 它返回一个简单的HTML表单，用于用户输入数据
#[get("/form")]
async fn form() -> impl Responder {
    "<html><body><form action='/submit' method='post'><input type='text' name='data'><button type='submit'>Submit</button></form></body></html>"
}

/// 这是一个处理POST请求的路由
/// 它接收用户输入并检测是否包含XSS攻击代码
/// 如果检测到XSS攻击代码，它将返回一个错误页面
/// 否则，它将显示一个安全的消息
#[post("/submit")]
async fn submit(data: web::Form<String>) -> impl Responder {
    let input = data.0;
    if detect_xss(&input) {
        return HttpResponse::BadRequest().body("XSS attack detected!")
    }
    let sanitized_input = sanitize_input(&input);
    HttpResponse::Ok().content_type("text/html").body(format!("<p>Submitted data: {}</p>", sanitized_input))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/form", get())
            .route("/submit", post())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
