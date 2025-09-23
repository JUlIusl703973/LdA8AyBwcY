use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

// 配置数据库连接池
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// 主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建环境变量
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建数据库连接池
    let pool = DbPool::builder()
        .max_size(15) // 设置最大连接数
        .build(ConnectionManager::<PgConnection>::new(database_url));

    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // 将数据库连接池添加到应用数据中
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 首页路由处理函数
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, database pool is working!")
}

// 数据库连接池的配置和实现
// 配置数据库连接池的结构体
#[derive(Clone)]
struct PoolData(DbPool);

// 定义数据库连接池的配置
pub struct DatabaseConfig {
    pub url: String,
}

// 实现DatabaseConfig的配置
impl DatabaseConfig {
    pub fn new(url: String) -> Self {
        DatabaseConfig { url }
    }
}

// 创建数据库连接池
impl PoolData {
    pub fn create_pool(config: &DatabaseConfig) -> DbPool {
        let manager = ConnectionManager::<PgConnection>::new(config.url.clone());
        r2d2::Pool::builder()
            .max_size(15) // 设置最大连接数
            .build(manager)
            .expect("Failed to create pool.")
    }
}

// 使用数据库连接池的示例
async fn use_database_pool(pool: web::Data<DbPool>) -> HttpResponse {
    // 从连接池中获取数据库连接
    let conn = pool.get().expect("Failed to get connection from pool.");

    // 在这里执行数据库操作...
    // 例如：插入、查询、更新等

    HttpResponse::Ok().body("Database operation is completed.")
}