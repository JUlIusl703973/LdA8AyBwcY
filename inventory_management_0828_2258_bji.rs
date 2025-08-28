use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// 定义库存项
#[derive(Serialize, Deserialize, Debug, Clone)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// 库存管理系统
struct InventoryManager {
    items: HashMap<u32, InventoryItem>,
}

impl InventoryManager {
    // 创建一个新的库存管理系统实例
    fn new() -> InventoryManager {
        InventoryManager {
            items: HashMap::new(),
        }
    }

    // 添加一个库存项
    fn add_item(&mut self, item: InventoryItem) {
        self.items.insert(item.id, item);
    }

    // 更新库存项的数量
    fn update_quantity(&mut self, item_id: u32, quantity: u32) -> Result<(), String> {
        if let Some(item) = self.items.get_mut(&item_id) {
            item.quantity = quantity;
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }

    // 获取库存项的数量
    fn get_quantity(&self, item_id: u32) -> Result<u32, String> {
        self.items.get(&item_id)
            .map(|item| item.quantity)
            .ok_or_else(|| "Item not found".to_string())
    }
}

// 定义API结构体
struct Api;

// 实现API端点
impl Api {
    #[post("/inventory")]
    async fn add_inventory_item(item: web::Json<InventoryItem>) -> impl Responder {
        let mut inventory = web::Data::get::<web::AppData<InventoryManager>>()
            .expect("Failed to get inventory manager")
            .clone();

        inventory.add_item(item.into_inner());

        HttpResponse::Ok().json(json!("Item added successfully"))
    }

    #[post("/update_quantity/{item_id}")]
    async fn update_inventory_quantity(item_id: web::Path<u32>, quantity: web::Json<u32>) -> impl Responder {
        let mut inventory = web::Data::get::<web::AppData<InventoryManager>>()
            .expect("Failed to get inventory manager")
            .clone();

        match inventory.update_quantity(*item_id, quantity.into_inner()) {
            Ok(_) => HttpResponse::Ok().json(json!("Quantity updated successfully")),
            Err(e) => HttpResponse::BadRequest().json(json!(e)),
        }
    }

    #[get("/quantity/{item_id}")]
    async fn get_inventory_quantity(item_id: web::Path<u32>) -> impl Responder {
        let inventory = web::Data::get::<web::AppData<InventoryManager>>()
            .expect("Failed to get inventory manager")
            .clone();

        match inventory.get_quantity(*item_id) {
            Ok(quantity) => HttpResponse::Ok().json(json!(quantity)),
            Err(e) => HttpResponse::BadRequest().json(json!(e)),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::AppData::new(InventoryManager::new()))
            .service(Api::add_inventory_item)
            .service(Api::update_inventory_quantity)
            .service(Api::get_inventory_quantity)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
