// cache_service.rs
// A cache service using the Actix framework

use actix::prelude::*;
use actix_web::{web, App, HttpServer, Responder};
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Define a cache item with expiration
struct CacheItem<T> {
    data: T,
    expires_at: Instant,
}

impl<T> CacheItem<T> {
    fn new(data: T, duration: Duration) -> Self {
        CacheItem {
            data,
            expires_at: Instant::now() + duration,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

// Cache service struct
struct CacheService {
    inner: Mutex<HashMap<String, CacheItem<String>>>,
}

impl CacheService {
    fn new() -> Self {
        CacheService {
            inner: Mutex::new(HashMap::new()),
        }
    }

    // Add a new item to the cache
    fn set(&self, key: String, value: String, duration: Duration) {
        let mut inner = self.inner.lock().unwrap();
        inner.insert(key, CacheItem::new(value, duration));
    }

    // Get an item from the cache
    fn get(&self, key: &str) -> Option<String> {
        let mut result = None;
        let inner = self.inner.lock().unwrap();
        if let Some(cache_item) = inner.get(key) {
            if !cache_item.is_expired() {
                result = Some(cache_item.data.clone());
            } else {
                // Remove expired item from cache
                inner.remove(key);
            }
        }
        result
    }
}

// Define a state for the cache service that can be shared across handlers
struct AppState {
    cache_service: CacheService,
}

// Implement a handler to set an item in the cache
async fn set_cache_item(state: web::Data<AppState>, web::Json(payload): web::Json<(String, String)>) -> impl Responder {
    let (key, value) = payload.into_inner();
    state.cache_service.set(key, value, Duration::from_secs(10)); // Cache for 10 seconds
    "Item cached for 10 seconds"
}

// Implement a handler to get an item from the cache
async fn get_cache_item(state: web::Data<AppState>, key: web::Path<String>) -> impl Responder {
    match state.cache_service.get(&key) {
        Some(value) => Ok(web::Json(value)),
        None => Ok("Item not found in cache"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState { cache_service: CacheService::new() }))
            .route("/set", web::post().to(set_cache_item))
            .route("/get/{key}", web::get().to(get_cache_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
