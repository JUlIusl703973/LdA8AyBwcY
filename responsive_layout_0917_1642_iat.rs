use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use serde_json::json;

/// 主程序入口，设置路由和启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = actix_web::App::new()
        .service(responsive_layout_controller)
        .configure(middleware);

    actix_web::HttpServer::new(move || app.clone())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/// 配置中间件
fn middleware(config: &mut web::ServiceConfig) {
    // 添加日志记录中间件
    config.wrap_fn(log::InfoLogger)
}

/// 响应式布局控制器
#[get("/responsive")]
async fn responsive_layout_controller() -> impl Responder {
    // 模拟响应式布局的数据
    let layout_data = json!({
        "header": "Header", 
        "footer": "Footer", 
        "content": "Hello, Actix!"
    });

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(layout_data.to_string())
}

/// 日志记录中间件
fn log(req: HttpRequest) -> (HttpRequest, MiddlewareResult) {
    println!("Request: {} {}", req.method(), req.uri());
    (req, MiddlewareResult::Done)
}

/// 中间件结果枚举
enum MiddlewareResult {
    Done,
}

/// HttpRequest 包装类型
struct HttpRequest;
