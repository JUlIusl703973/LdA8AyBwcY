// Restful API using Rust and Actix framework
// This file provides a basic example of a RESTful API service.

use actix_web::{
    get,
    HttpResponse,
    web,
    App,
    HttpServer,
    Responder,
};
use serde::{Serialize, Deserialize};
use serde_json::json;

// Define a simple data structure to represent a User.
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
}

// Define a handler function for GET requests to retrieve a user.
#[get("/user/{id}")]
async fn get_user(id: web::Path<u32>) -> impl Responder {
    // Simulate fetching a user from a database.
    let user = User {
        id: id.into_inner(),
        name: "John Doe".to_string(),
    };

    // Return the user as a JSON response.
    HttpResponse::Ok().json(user)
}

// Define a handler function for GET requests to retrieve all users.
#[get("/users")]
async fn get_users() -> impl Responder {
    // Simulate fetching all users from a database.
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ];

    // Return the users as a JSON response.
    HttpResponse::Ok().json(users)
}

// Define the main function, which sets up the Actix web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server with the configured routes.
    HttpServer::new(||
        App::new()
            // Register the handler functions for the routes.
            .service(get_user)
            .service(get_users)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
