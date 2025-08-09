use actix_web::{get, HttpResponse, Responder, web};
use sysinfo::{System, SystemExt};

/// 系统性能监控工具
/// 提供一个HTTP接口，用于获取系统性能数据
#[get("/system")]
async fn system_info() -> impl Responder {
    let mut sys = System::new_all(); // 获取系统信息
    sys.refresh_all(); // 刷新系统信息

    // 构建响应数据
    let response_data = format!("
    {{
        "cpu_usage": {sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect::<Vec<_>>()[0]},
        "ram_usage": {sys.used_memory()},
        "total_ram": {sys.total_memory()},
        "disk_usage": {sys.disks().iter().map(|d| d.free()).collect::<Vec<_>>()[0] * d.size() / 100.0}
    }}
    ");

    // 返回JSON响应
    HttpResponse::Ok().content_type("application/json").body(response_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置Actix Web服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(system_info) // 注册系统性能监控接口
    })
    .bind("127.0.0.1:8080")? // 绑定地址和端口
    .run()
    .await
}