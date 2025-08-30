use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as _};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 模拟数据库连接池
static DB_POOL: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(vec![1, 2, 3, 4, 5]));

#[get("/query_optimizer