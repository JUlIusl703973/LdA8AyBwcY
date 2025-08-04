use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use r2d2_diesel::ConnectionManager as DieselConnectionManager;

// 定义数据库连接池
pub struct ApiConfig {
    pub pool: r2d2::Pool<DieselConnectionManager<PgConnection>>,
}

// 异步数据库连接
pub async fn db_connection(app_config: web::Data<ApiConfig>) -> Result<PgConnection, diesel::r2d2::Error> {
    app_config.pool.get().map_err(|e| e.into())
}

// 定义数据库连接池的配置
pub fn database_config() -> Result<ApiConfig, diesel::r2d2::Error> {
    let database_url = 