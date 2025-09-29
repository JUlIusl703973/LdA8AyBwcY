use actix_web::{get, HttpResponse, Responder, web};

// 主程序结构体
struct AppState {}

// HTTP请求处理器
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// 配置Actix Web应用
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务器
    actix_web::HttpServer::new(|| {
        // 将handler添加到服务器
        actix_web::App::new()
            .app_data(AppState::default()) // 应用状态
            .service(index) // 将index handler添加到服务器
    })
    .bind("127.0.0.1:8080")? // 绑定地址和端口
    .run()
    .await
}

// 实现AppState的Default trait
impl Default for AppState {
    fn default() -> Self {
        AppState {}
    }
}