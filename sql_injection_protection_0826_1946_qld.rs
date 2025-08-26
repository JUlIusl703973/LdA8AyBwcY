use actix_web::{get, HttpResponse, Responder, post, web, App, HttpServer, Responder as ActixResponder};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::mysql::MysqlConnection;
use serde::Deserialize;

// 定义数据库连接的类型，这里以PostgreSQL为例
type DbPool = diesel::r2d2::Pool<diesel::r2d2_diesel::ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct QueryParams {
    // 使用serde的Deserialize特性来解析HTTP请求参数
    search: String,
}

// SQL注入防护的函数，使用参数化查询
#[get(