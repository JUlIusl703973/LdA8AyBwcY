use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

// 定义数据库配置结构体
struct PgPool(r2d2::Pool<ConnectionManager<PgConnection>>);

// 实现Send和Sync特性，以便可以在异步上下文中安全地共享和使用数据库连接池
impl PgPool {
    pub fn new() -> r2d2::Pool<ConnectionManager<PgConnection>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    // 获取数据库连接的异步方法
    pub async fn get_conn(&self) -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
        self.0.get().expect("Failed to get DB connection from pool")
    }
}

#[get("/")]
async fn index(pool: web::Data<PgPool>) -> impl Responder {
    let conn = pool.get_conn().await;
    // 使用prepared statements来防止SQL注入
    let results = conn
        .query("SELECT * FROM users WHERE username = $1", &[web::Query::from_str("username").await.unwrap_or("").into()])
        .expect("Error loading data");

    // 将查询结果转换为JSON响应
    let users: Vec<_> = results.iter().map(|user| user.0).collect();
    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志记录器
    env_logger::init();

    // 创建数据库连接池
    let pool = web::Data::new(PgPool::new());

    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}