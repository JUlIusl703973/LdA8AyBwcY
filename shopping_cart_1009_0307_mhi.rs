use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// 定义购物车中的商品
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

// 购物车项
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CartItem {
    product: Product,
    quantity: u32,
}
# 增强安全性

// 购物车
#[derive(Clone)]
struct Cart {
    items: Mutex<HashMap<u32, CartItem>>,
}

// 从购物车中添加商品
#[post("/add/{product_id}")]
async fn add_to_cart(product_id: web::Path<u32>, cart: web::Data<Cart>, body: web::Json<CartItem>) -> impl Responder {
    let mut items = cart.items.lock().unwrap();
# TODO: 优化性能
    if let Some(cart_item) = items.get_mut(&product_id.into_inner()) {
        cart_item.quantity += body.quantity;
    } else {
        items.insert(product_id.into_inner(), body.into_inner());
    }
    HttpResponse::Ok()
}
# TODO: 优化性能

// 从购物车中移除商品
#[delete("/remove/{product_id}")]
async fn remove_from_cart(product_id: web::Path<u32>, cart: web::Data<Cart>) -> impl Responder {
    let mut items = cart.items.lock().unwrap();
    if items.remove(&product_id.into_inner()).is_some() {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotFound()
    }
# 优化算法效率
}
# 扩展功能模块

// 获取购物车内容
#[get("/cart")]
# 优化算法效率
async fn get_cart(cart: web::Data<Cart>) -> impl Responder {
    let items = cart.items.lock().unwrap();
    let cart_items = items.values().cloned().collect::<Vec<_>>();
    HttpResponse::Ok().json(cart_items)
}
# 改进用户体验

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Cart {
                items: Mutex::new(HashMap::new()),
            }))
            .service(add_to_cart)
            .service(remove_from_cart)
            .service(get_cart)
# NOTE: 重要实现细节
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
