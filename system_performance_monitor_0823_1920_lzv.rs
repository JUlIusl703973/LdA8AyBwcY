use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use sysinfo::{System, SystemExt};
use std::sync::Arc;

/// 系统性能监控工具
#[derive(Clone)]
struct SystemPerformance {
    system: Arc<System>,
}

impl SystemPerformance {
    /// 创建一个新的系统性能监控工具实例
    pub fn new() -> Self {
        SystemPerformance {
            system: Arc::new(System::new_all()),
        }
    }

    /// 获取系统性能状态
    async fn get_system_status(&self) -> Result<impl Responder, HttpResponse> {
        self.system.refresh_all();
        let system_status = SystemStatus {
            cpu_usage: self.system.cpus().iter()
                .map(|cpu| cpu.cpu_usage() as f64)
                .collect::<Vec<f64>>(),
            memory_usage: self.system.used_memory() as f64,
            disk_usage: self.system.disks().iter()
                .map(|disk| disk.free_space())
                .collect::<Vec<u64>>(),
        };
        Ok(HttpResponse::Ok().json(system_status))
    }
}

/// 系统状态结构体
#[derive(serde::Serialize, serde::Deserialize)]
struct SystemStatus {
    cpu_usage: Vec<f64>,
    memory_usage: f64,
    disk_usage: Vec<u64>,
}

/// 系统性能监控服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/status")
                .route(web::get().to(get_system_status))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// 获取系统性能状态的HTTP处理器
#[get("/status")]
async fn get_system_status(service: web::Data<SystemPerformance>) -> impl Responder {
    service.get_system_status().await
}
