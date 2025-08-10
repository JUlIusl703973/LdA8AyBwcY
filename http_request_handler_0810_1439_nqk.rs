use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// Handler for the root path.
///
/// This function returns a simple "Hello, world!" response.
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

/// Start the HTTP server.
///
/// This function starts an HTTP server with the root path handler.
/// It listens on the specified address and port.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // Register the index handler.
    })
    .bind("127.0.0.1:8080")? // Bind to the specified address and port.
    .run()
    .await
}
