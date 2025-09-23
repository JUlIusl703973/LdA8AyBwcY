use actix_web::{get, HttpResponse, Responder};
use sys_info;
use std::sync::Arc;

// 定义服务结构体
struct MemoryService;

// 定义内存使用情况的响应结构体
#[derive(Debug, Serialize)]
struct MemoryUsage {
    total_memory: u64,
    available_memory: u64,
    used_memory: u64,
    used_memory_percent: f32,
}

// 实现MemoryService
impl MemoryService {
    #[get("/memory")]
    async fn memory_usage(&self) -> impl Responder {
        // 尝试获取内存信息
        match sys_info::mem_info() {
            Ok(mem_info) => {
                // 计算内存使用百分比
                let used_memory_percent = 100.0 * (mem_info.total - mem_info.avail) as f32 / mem_info.total as f32;

                // 创建响应结构体并返回
                HttpResponse::Ok().json(MemoryUsage {
                    total_memory: mem_info.total,
                    available_memory: mem_info.avail,
                    used_memory: mem_info.total - mem_info.avail,
                    used_memory_percent,
                })
            },
            Err(e) => {
                // 错误处理
                HttpResponse::InternalServerError().body(e.to_string())
            },
        }
    }
}

// 定义Actix Web服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 注册MemoryService
            .service(MemoryService::memory_usage)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 以下是Cargo.toml依赖项
// [dependencies]
// actix-web = "4.0"
// sys-info = "0.9"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
