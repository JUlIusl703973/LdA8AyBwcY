// config_manager.rs
// 这是一个使用Rust和Actix框架的配置文件管理器

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use serde::Deserialize;
use serde_json::json;

// 定义配置数据结构
#[derive(Deserialize, Debug)]
struct AppConfig {
    database_url: String,
    api_key: String,
}

// 配置文件管理器
struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    // 初始化配置文件管理器
    pub fn new(config_path: &str) -> Result<Self, io::Error> {
        // 读取配置文件
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // 解析配置文件
        let config: AppConfig = serde_json::from_str(&contents)?;
        
        Ok(ConfigManager { config })
    }

    // 获取配置项
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }
}

// 实现HTTP服务
async fn get_config_data(cfg: web::Data<HashMap<String, String>>) -> impl Responder {
    // 从共享数据中获取配置
    let config_data = cfg.get("database_url").unwrap_or(&"N/A".to_string());
    HttpResponse::Ok().json(json!({
        "database_url": config_data,
        "api_key": cfg.get("api_key").unwrap_or(&"N/A".to_string()),
    }))
}

fn main() -> io::Result<()> {
    // 初始化配置文件管理器
    let config_manager = ConfigManager::new("config.json").expect("Failed to load config file");
    
    // 将配置项存储到共享的HashMap中
    let config_data = web::Data::new(
        HashMap::from([("database_url".to_string(), config_manager.get_config().database_url.clone()), ("api_key".to_string(), config_manager.get_config().api_key.clone())])
    );
    
    // 设置Actix Web服务器
    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .service(web::resource("/config").route(get().to(get_config_data)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
