use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

/// User model represents a user with their permissions.
#[derive(Debug, Deserialize)]
struct User {
    id: u32,
    username: String,
    permissions: Vec<String>,
}

/// PermissionService handles user permission checks.
struct PermissionService;

impl PermissionService {
    /// Check if the user has the specified permission.
    #[allow(dead_code)]
    fn has_permission(&self, user: &User, permission: &str) -> bool {
        user.permissions.contains(&permission.to_string())
    }
}

/// The main application handler.
struct AppState {
    service: PermissionService,
    users: HashMap<u32, User>,
}

/// Define the route for checking user permissions.
#[get("/check_permission/{user_id}/{permission}")]
async fn check_permission(state: web::Data<AppState>, user_id: web::Path<u32>, permission: web::Path<String>) -> impl Responder {
    let user = state.users.get(&user_id.into_inner()).unwrap_or_else(|| {
        panic!("User not found");
    });
    
    if state.service.has_permission(user, &permission.into_inner()) {
        HttpResponse::Ok().json(json!({"message": "Permission granted"}))
    } else {
        HttpResponse::Forbidden().json(json!({"message": "Permission denied"}))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the AppState with a PermissionService and a sample user map.
    let app_state = web::Data::new(AppState {
        service: PermissionService,
        users: HashMap::from([
            (1, User { id: 1, username: "alice".to_string(), permissions: vec!["read", "write"] }),
            (2, User { id: 2, username: "bob".to_string(), permissions: vec!["read"] }),
        ]),
    });

    // Configure the server with the state and route.
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(check_permission)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
