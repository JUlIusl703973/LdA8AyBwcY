use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use diesel::prelude::*;
use diesel::migration::{self, MigrationDirection};
use dotenv::dotenv;
use std::env;

// 定义数据库配置结构体
#[derive(Debug)]
struct DbConfig {
    database_url: String,
}

// 定义迁移模块
mod migrations {
    // 在此处添加数据库迁移文件
    // 使用 Diesel CLI 创建迁移文件
    // 例如：diesel migration generate create_users
    // 然后在该模块中添加迁移文件
    // 例如：include!("./migrations/2023-04-01-000001_create_users/up.sql");
}

// 定义错误处理类型
#[derive(Debug)]
enum MigrationError {
    DieselError(diesel::result::Error),
    IoError(std::io::Error),
}

impl From<diesel::result::Error> for MigrationError {
    fn from(err: diesel::result::Error) -> MigrationError {
        MigrationError::DieselError(err)
    }
}

impl From<std::io::Error> for MigrationError {
    fn from(err: std::io::Error) -> MigrationError {
        MigrationError::IoError(err)
    }
}

// 定义迁移执行函数
fn execute_migration(db_url: &str, direction: MigrationDirection) -> Result<(), MigrationError> {
    let connection = PgConnection::establish(db_url)
        .map_err(MigrationError::from)?;

    migration::run(
        &connection,
        direction,
        &migrations::MIGRATIONS.iter().collect::<Vec<_>>(),
    )
    .map_err(MigrationError::from)
}

// 定义迁移路由
async fn migrate_web_route(db_config: web::Data<DbConfig>) -> impl Responder {
    let direction = web::Query::<HashMap<String, String>>::from_query(