use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::collections::HashMap;

// 定义一个结构体来存储统计数据
#[derive(Default)]
struct DataAnalysisService {
    data: HashMap<String, i32>,
}

impl DataAnalysisService {
    // 初始化一个新的DataAnalysisService实例
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // 添加数据项
    pub fn add_data(&mut self, key: String, value: i32) {
        let count = self.data.entry(key).or_insert(0);
        *count += value;
    }

    // 获取数据项的计数
    pub fn get_data(&self, key: &str) -> Option<i32> {
        self.data.get(key).cloned()
    }

    // 清除所有数据
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

// 定义一个Actix服务
struct DataAnalysisServiceServer;

impl DataAnalysisServiceServer {
    // 创建一个新的服务实例
    pub fn new() -> Self {
        Self {}
    }

    // 添加数据项的API
    pub async fn add_data_item(
        &self,
        web::Path((key,)): web::Path<(String,)>,
        body: web::Json<HashMap<String, i32>>,
        service: web::Data<DataAnalysisService>,
    ) -> impl Responder {
        service.add_data(key, *body.get("value").unwrap_or(&0));
        HttpResponse::Ok().json({"status": "data added"})
    }

    // 获取数据项的计数的API
    pub async fn get_data_item(
        &self,
        web::Path(key): web::Path<String>,
        service: web::Data<DataAnalysisService>,
    ) -> impl Responder {
        match service.get_data(&key) {
            Some(value) => HttpResponse::Ok().json({"key": key, "value": value}),
            None => HttpResponse::NotFound().json({"error": "data not found"}),
        }
    }

    // 清除所有数据的API
    pub async fn clear_data(&self, service: web::Data<DataAnalysisService>) -> impl Responder {
        service.clear_data();
        HttpResponse::Ok().json({"status": "data cleared"})
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建服务实例
    let service = DataAnalysisService::new();
    
    HttpServer::new(move || {
        App::new()
            // 添加API路由
            .service(web::resource("/add").route(web::post().to(DataAnalysisServiceServer::add_data_item))
            .service(web::resource("/get/{key}").route(web::get().to(DataAnalysisServiceServer::get_data_item)))
            .service(web::resource("/clear").route(web::post().to(DataAnalysisServiceServer::clear_data)))
            // 将服务实例存储在App的数据中
            .app_data(web::Data::new(service))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
