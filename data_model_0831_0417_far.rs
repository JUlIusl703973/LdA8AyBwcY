use actix_web::{get, post, put, delete, web, HttpResponse, Responder, http::StatusCode, App, HttpServer, Responder as HttpResponseBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::collections::HashMap;

// Define a data model for our application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

// In-memory data store for demonstration purposes
lazy_static::lazy_static! {
    static ref USERS: Mutex<HashMap<u32, User>> = Mutex::new(HashMap::new());
}

// Helper function to generate a new ID for a user
fn generate_id() -> u32 {
    let mut users = USERS.lock().unwrap();
    let max_id = users.keys().max().unwrap_or(&0).clone();
    max_id + 1
}

// Handler for getting a user by ID
#[get("/users/{id}")]
async fn get_user(id: web::Path<u32>) -> impl Responder {
    let users = USERS.lock().unwrap();
    match users.get(&id.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(json!({ 