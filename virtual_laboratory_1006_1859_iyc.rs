use actix_web::{web, App, HttpServer, Responder, get, HttpResponse, error::ErrorUnauthorized};
use actix_web::middleware::Logger;
use std::collections::HashMap;
# NOTE: 重要实现细节
use serde::Serialize;
use serde_json::json;

// 定义一个结构体来表示实验室的状态
#[derive(Serialize, Debug)]
# 添加错误处理
struct Laboratory {
    id: u32,
    name: String,
    status: String,
}

// 定义实验室的状态
impl Laboratory {
    fn new(id: u32, name: &str) -> Self {
        Laboratory {
# 优化算法效率
            id,
            name: name.to_string(),
            status: "inactive".to_string(), // 初始状态为 inactive
        }
    }
}

// 定义一个模拟实验室的HashMap
struct VirtualLaboratory {
    labs: HashMap<u32, Laboratory>,
}

impl VirtualLaboratory {
    fn new() -> Self {
        VirtualLaboratory {
            labs: HashMap::new(),
        }
# 改进用户体验
    }

    // 创建一个实验室
# 改进用户体验
    fn create_lab(&mut self, id: u32, name: &str) -> Result<&str, &str> {
        if self.labs.contains_key(&id) {
# 扩展功能模块
            Err("Lab already exists")
# FIXME: 处理边界情况
        } else {
# 优化算法效率
            self.labs.insert(id, Laboratory::new(id, name));
            Ok("Lab created successfully")
        }
    }
# 改进用户体验

    // 获取实验室的状态
    fn get_lab_status(&self, id: u32) -> Result<Laboratory, &str> {
        self.labs.get(&id).cloned().ok_or("Lab not found")
    }
}
# NOTE: 重要实现细节

// 实现 Actix 路由处理器
async fn get_lab_status_handler(lab: web::Data<VirtualLaboratory>, id: web::Path<u32>) -> impl Responder {
    match lab.get_lab_status(id.into_inner()) {
        Ok(lab) => HttpResponse::Ok().json(lab),
# TODO: 优化性能
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": e,
        })),
    }
}
# 扩展功能模块

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let lab = web::Data::new(VirtualLaboratory::new());
        App::new()
            .wrap(Logger::default())
            .data(lab)
            .route("/lab/{id}", get().to(get_lab_status_handler))
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().finish()
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
# NOTE: 重要实现细节
    .await
}
