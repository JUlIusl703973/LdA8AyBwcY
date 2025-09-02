use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;

// 定义数据库连接池
struct PgPool(Pool<ConnectionManager<PgConnection>>);
impl PgPool {
    fn get(&self) -> Result<PgConnection, diesel::r2d2::Error> {
        self.0.get()
    }
}

// 定义查询请求数据结构
#[derive(Deserialize)]
struct QueryParams {
    search: String,
}

// 定义防止SQL注入的函数
#[get("/search")]
async fn search(db_pool: web::Data<PgPool>, params: web::Query<QueryParams>) -> impl Responder {
    let conn = match db_pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // 使用参数化查询防止SQL注入
    let results = match diesel::sql_query("SELECT * FROM my_table WHERE my_column = $1")
        .bind::<diesel::sql_types::Text, _>(&params.search)
        .load::<YourModel>(&conn) {
         Ok(results) => results,
         Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 建立数据库连接池
    let manager = ConnectionManager::<PgConnection>::new(
        "postgres://username:password@localhost/dbname"
    );
    let pool = Pool::builder()
        .build(manager)
        .unwrap();
    let db_pool = web::Data::new(PgPool(pool));

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .route("/search", web::get().to(search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}