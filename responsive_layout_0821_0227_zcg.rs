use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::Deserialize;
use serde_json::json;

// 定义一个请求体结构，用于接收前端发送的屏幕尺寸信息
#[derive(Deserialize)]
pub struct ScreenSize {
    width: u32,
    height: u32,
}

// 定义一个响应结构，用于向前端返回响应式布局信息
#[derive(Serialize)]
pub struct ResponsiveLayout {
    layout: String,
}

// 创建一个响应式布局的服务
#[get("/layout")]
async fn responsive_layout(data: web::Json<ScreenSize>) -> impl Responder {
    // 根据屏幕尺寸确定布局类型
    let layout = match (data.width, data.height) {
        // 如果宽度大于高度，认为是横向布局
        (width, height) if width > height => "landscape".to_string(),
        // 如果高度大于宽度，认为是纵向布局
        _ => "portrait".to_string(),
    };

    // 返回响应式布局信息
    HttpResponse::Ok().json(ResponsiveLayout { layout })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            // 注册服务
            .service(responsive_layout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
