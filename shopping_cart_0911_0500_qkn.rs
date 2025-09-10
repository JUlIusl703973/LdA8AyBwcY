use actix_web::{get, post, put, delete, web, HttpResponse, Responder, App, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义购物车中的商品项
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    id: u32,
    name: String,
    quantity: u32,
    price: f64,
}

// 定义购物车结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ShoppingCart {
    items: Vec<CartItem>,
}

// 全局购物车数据，使用Mutex来实现线程安全
lazy_static::lazy_static! {
    static ref GLOBAL_CART: Mutex<HashMap<u32, ShoppingCart>> = Mutex::new(HashMap::new());
}

// 获取购物车的路由
#[get("/cart/{user_id}")]
async fn get_cart(user_id: web::Path<u32>) -> impl Responder {
    let cart = GLOBAL_CART.lock().unwrap();
    match cart.get(&user_id.into_inner()) {
        Some(cart) => HttpResponse::Ok().json(cart),
        None => HttpResponse::NotFound().json(json!{"error": "Cart not found"}),
    }
}

// 添加商品到购物车的路由
#[post("/cart/{user_id}")]
async fn add_item(user_id: web::Path<u32>, item: web::Json<CartItem>) -> impl Responder {
    let mut cart = GLOBAL_CART.lock().unwrap();
    let user_id = user_id.into_inner();
    let mut cart_item = CartItem {
        id: item.id,
        name: item.name.clone(),
        quantity: 1,
        price: item.price,
    };
    match cart.get_mut(&user_id) {
        Some(cart) => {
            if let Some(existing_item) = cart.items.iter_mut().find(|i| i.id == item.id) {
                existing_item.quantity += 1;
            } else {
                cart.items.push(cart_item.clone());
            }
        },
        None => {
            let mut new_cart = ShoppingCart { items: vec![cart_item] };
            cart.insert(user_id, new_cart);
        },
    }
    HttpResponse::Ok().json(cart_item)
}

// 更新购物车中的商品数量的路由
#[put("/cart/{user_id}/{item_id}")]
async fn update_item(user_id: web::Path<u32>, item_id: web::Path<u32>, quantity: web::Json<u32>) -> impl Responder {
    let mut cart = GLOBAL_CART.lock().unwrap();
    let user_id = user_id.into_inner();
    let item_id = item_id.into_inner();
    if let Some(cart) = cart.get_mut(&user_id) {
        if let Some(item) = cart.items.iter_mut().find(|i| i.id == item_id) {
            item.quantity = quantity.into_inner();
            HttpResponse::Ok().json(item)
        } else {
            HttpResponse::NotFound().json(json!{"error": "Item not found"})
        }
    } else {
        HttpResponse::NotFound().json(json!{"error": "Cart not found"})
    }
}

// 从购物车中删除商品的路由
#[delete("/cart/{user_id}/{item_id}")]
async fn remove_item(user_id: web::Path<u32>, item_id: web::Path<u32>) -> impl Responder {
    let mut cart = GLOBAL_CART.lock().unwrap();
    let user_id = user_id.into_inner();
    let item_id = item_id.into_inner();
    if let Some(cart) = cart.get_mut(&user_id) {
        if let Some(pos) = cart.items.iter().position(|i| i.id == item_id) {
            cart.items.remove(pos);
            HttpResponse::Ok().json(json!{"message": "Item removed"})
        } else {
            HttpResponse::NotFound().json(json!{"error": "Item not found"})
        }
    } else {
        HttpResponse::NotFound().json(json!{"error": "Cart not found"})
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_cart)
            .service(add_item)
            .service(update_item)
            .service(remove_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 引入必要的库
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate actix_web;

// 为全局变量定义lazy_static宏
#[macro_use]
extern crate lazy_static;

// 定义全局购物车数据宏
lazy_static::lazy_static! {
    static ref GLOBAL_CART: Mutex<HashMap<u32, ShoppingCart>> = Mutex::new(HashMap::new());
}

// 引入lazy_static库
extern crate lazy_static;

// 引入actix_web库
extern crate actix_web;

// 引入serde库
extern crate serde;

// 引入serde_json库
extern crate serde_json;


/// 获取购物车信息的函数
///
/// 根据用户ID获取购物车信息
///
/// # Arguments
///
/// * `user_id` - 用户ID
///
/// # Returns
///
/// * `HttpResponse` - 购物车信息或者错误信息
///
#[get("/cart/{user_id}")]
async fn get_cart(user_id: web::Path<u32>) -> impl Responder {
    // ...
}

/// 添加商品到购物车
///
/// 根据用户ID和商品项信息添加商品到购物车
///
/// # Arguments
///
/// * `user_id` - 用户ID
/// * `item` - 商品项信息
///
/// # Returns
///
/// * `HttpResponse` - 添加的商品项或者错误信息
///
#[post("/cart/{user_id}")]
async fn add_item(user_id: web::Path<u32>, item: web::Json<CartItem>) -> impl Responder {
    // ...
}

/// 更新购物车中的商品数量
///
/// 根据用户ID和商品ID更新购物车中的商品数量
///
/// # Arguments
///
/// * `user_id` - 用户ID
/// * `item_id` - 商品ID
/// * `quantity` - 新的商品数量
///
/// # Returns
///
/// * `HttpResponse` - 更新的商品项或者错误信息
///
#[put("/cart/{user_id}/{item_id}")]
async fn update_item(user_id: web::Path<u32>, item_id: web::Path<u32>, quantity: web::Json<u32>) -> impl Responder {
    // ...
}

/// 从购物车中删除商品
///
/// 根据用户ID和商品ID从购物车中删除商品
///
/// # Arguments
///
/// * `user_id` - 用户ID
/// * `item_id` - 商品ID
///
/// # Returns
///
/// * `HttpResponse` - 操作结果或者错误信息
///
#[delete("/cart/{user_id}/{item_id}")]
async fn remove_item(user_id: web::Path<u32>, item_id: web::Path<u32>) -> impl Responder {
    // ...
}
