use actix_web::{get, HttpResponse, Responder, web};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::sql_query;

/// Database connection trait object
type DbPool = web::Data<SqlitePool>;

/// Home handler
#[get("/")]
async fn home(db_pool: DbPool) -> impl Responder {
    let conn = db_pool.get().expect("couldn't get db connection from pool");

    // Prevent SQL injection by using parameterized queries
    let user_input = "some_user_input"; // This should come from a request parameter
    let query = sql_query("SELECT * FROM users WHERE username = ?").bind(user_input);

    match query.load::<(u32, String)>(conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error loading users: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Setup the application with a database connection pool
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the database connection pool
    let database_url = "sqlite:mydatabase.db";
    let pool = SqlitePool::connect(database_url).expect("Failed to create pool.");

    // Set up the Actix web server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(pool.clone())) // Move clone into the handler
            .service(home)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}