#[macro_use]
extern crate actix_web;
use actix_web::{get,
              HttpResponse,
# FIXME: 处理边界情况
              Responder};
use serde::Serialize;
use serde_json::json;

// 定义响应式布局状态
#[derive(Serialize)]
struct ResponsiveLayout {
    width: u32,
    height: u32,
    is_mobile: bool,
}

// 主控制器
# 改进用户体验
struct LayoutController;

// 使用get宏定义路由和处理函数
#[get("/layout/{width}/{height}")]
async fn layout_layout_controller(
    width: u32,
    height: u32,
) -> impl Responder {
    // 根据屏幕宽高判断是否为移动设备
    let is_mobile = width < 768 && height < 768;

    // 创建响应式布局状态
    let state = ResponsiveLayout {
        width,
        height,
        is_mobile,
    };

    // 返回JSON响应
    json!(state)
# 增强安全性
}
# 改进用户体验

// 应用启动函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 注册路由
            .service(layout_layout_controller)
    })
# TODO: 优化性能
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
