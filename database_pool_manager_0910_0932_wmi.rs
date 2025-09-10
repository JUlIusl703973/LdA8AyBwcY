use actix_web::web;
use actix_web::{App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;
use r2d2_diesel::DieselConnectionManager;

// 定义R2D2数据库连接池配置
#[derive(Clone)]
struct DatabasePool(r2d2::Pool<DieselConnectionManager<PgConnection>>);

impl DatabasePool {
    fn new(database_url: &str) -> DatabasePool {
        let manager = DieselConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        DatabasePool(pool)
    }
}

// 定义配置结构体
#[derive(Deserialize, Clone)]
struct Config {
    database_url: String,
}

// 定义数据库池配置服务
async fn get_database_pool() -> DatabasePool {
    let config: Config = Config::new();
    DatabasePool::new(&config.database_url)
}

// 主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config: Config = Config::new();
    
    // 初始化数据库连接池
    let pool = get_database_pool().await;
    
    // 设置Actix服务配置
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/").to(|| async {
                Ok("Hello world!")
            })).
            service(web::resource("/database").to(|| async {
                let pool = web::Data::get::<web::Data<DatabasePool>>()
                    .expect("Failed to get database pool.")
                    .0.clone();
                let conn = pool.get().expect("Failed to get connection from pool.");
                // 在这里执行数据库操作...
                Ok("Database connection established.")
            }))
    }).
   .bind("0.0.0.0:8080")?
   .run()
   .await
}

// Config结构体实现new方法用于获取环境变量中的配置
impl Config {
    fn new() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        }
    }
}

// 程序错误处理
#[derive(Debug)]
enum AppError {
    DbError(diesel::result::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppError::DbError(ref err) => write!(f, "Database error: {}", err),
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match *self {
            AppError::DbError(_) =>
                actix_web::HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error"
                })).
        }
    }
}
