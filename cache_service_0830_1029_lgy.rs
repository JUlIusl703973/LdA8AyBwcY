use actix_web::{web, App, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use std::thread;
use std::time::SystemTime;
use std::sync::Arc;

// 定义缓存项
struct CacheItem<T> {
    data: T,
    expiry: SystemTime,
}

// 缓存服务
struct CacheService<T> {
    cache: Mutex<HashMap<String, CacheItem<T>>>,
}

impl<T> CacheService<T> {
    // 创建一个新的缓存服务
    fn new() -> Self {
        CacheService {
            cache: Mutex::new(HashMap::new()),
        }
    }

    // 设置缓存项
    fn set(&self, key: String, data: T, duration: Duration) -> Result<(), String> {
        let mut cache = self.cache.lock().map_err(|e| e.to_string())?;
        let expiry = SystemTime::now() + duration;
        cache.insert(key, CacheItem { data, expiry });
        Ok(())
    }

    // 获取缓存项
    fn get(&self, key: &str) -> Option<T> {
        let cache = self.cache.lock().map_err(|_| ())?;
        cache.get(key)
            .and_then(|item| {
                if item.expiry > SystemTime::now() {
                    Some(item.data.clone())
                } else {
                    None
                }
            })
    }
}

// 定义应用状态
struct AppState {
    cache_service: Arc<CacheService<String>>,
}

// 定义HTTP Handler
async fn cache_handler(state: web::Data<AppState>, key: web::Path<String>) -> impl Responder {
    match state.cache_service.get(&key) {
        Some(value) => Ok(format!("Cache HIT for {}: {}", key, value)),
        None => Ok(format!("Cache MISS for {}", key)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建缓存服务
    let cache_service = CacheService::new();
    
    // 将缓存服务放入应用状态
    let state = AppState {
        cache_service: Arc::new(cache_service),
    };
    
    // 创建HTTP服务器并配置路由
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/cache/{key}", web::get().to(cache_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}