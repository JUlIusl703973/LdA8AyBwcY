use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

// 定义连接池
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// 定义数据库模式
#[derive(Queryable)]
struct QueryPlan {
    id: i32,
    plan_text: String,
}

// SQL查询优化器
#[get(