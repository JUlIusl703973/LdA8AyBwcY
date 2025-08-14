use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use toml;

// Define a struct to represent configuration settings
#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
    // Additional configuration fields can be added here
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
}

// Define a struct to handle configuration loading
struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    // Loads the configuration from a TOML file
    pub fn new(config_path: &str) -> io::Result<ConfigManager> {
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the contents of the file as TOML into the Config struct
        let config: Config = toml::from_str(&contents)?;
        Ok(ConfigManager { config })
    }

    // Retrieves a value from the database configuration
    pub fn get_database_config(&self) -> &DatabaseConfig {
        &self.config.database
    }
}

// Define the endpoint to retrieve the database configuration
async fn get_config(cfg: web::Data<ConfigManager>) -> impl Responder {
    match cfg.get_database_config() {
        Some(db_config) => HttpResponse::Ok().json(db_config),
        None => HttpResponse::InternalServerError().body(