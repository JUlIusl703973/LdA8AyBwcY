// process_manager.rs
//
// A simple process manager using Rust and Actix framework.

use actix_web::{
    get,
    web,
    App,
    HttpServer,
    Responde,
    Error,
};
use std::process::Command;
use serde::Serialize;
use serde_json::json;
use std::io::{self, Write};

// Define a struct to hold process information.
#[derive(Serialize)]
struct ProcessInfo {
    name: String,
    pid: u32,
}

// Define a struct to hold the result of a process execution.
#[derive(Serialize)]
struct ProcessExecutionResult {
    stdout: String,
    stderr: String,
}

// Define an enum to handle different process management operations.
enum ProcessOperation {
    Start,
    Stop,
    List,
}

// Define an error type for process management operations.
#[derive(Debug)]
enum ProcessManagerError {
    CommandExecutionFailed(String),
    InvalidOperation,
}

// Implement error conversion for ProcessManagerError.
impl From<ProcessManagerError> for actix_web::Error {
    fn from(err: ProcessManagerError) -> Self {
        actix_web::Error::from_repr(err.to_string())
    }
}

// Define a handler for listing processes.
#[get("/processes")]
async fn list_processes() -> Result<impl Responde, Error> {
    let mut processes = vec![];
    for pid in 1..10000 {
        if let Ok(name) = Command::new("ps")
            .args(&[ "-p", &pid.to_string(), "-o", "comm=" ])
            .output() {
            if let Ok(name) = String::from_utf8(name.stdout) {
                let name = name.trim().to_string();
                processes.push(ProcessInfo { name, pid: pid as u32 });
            }
        }
    }
    Ok(json!(processes))
}

// Define a handler for starting a process.
#[get("/process/start/{operation}")]
async fn start_process(operation: web::Path<String>) -> Result<impl Responde, Error> {
    match operation.into_inner().as_str() {
        "list" => Ok(json!{"message": "Process started successfully"}
            .to_string()
            .into()),
        _ => Err(ProcessManagerError::InvalidOperation.into()),
    }
}

// Define a handler for stopping a process.
#[get("/process/stop/{pid}")]
async fn stop_process(pid: web::Path<u32>) -> Result<impl Responde, Error> {
    let result = Command::new("kill")
        .arg(pid.into_inner().to_string())
        .output();
    match result {
        Ok(output) if output.status.success() => Ok(json!{"message": "Process stopped successfully"}
            .to_string()
            .into()),
        Ok(_) => Err(ProcessManagerError::CommandExecutionFailed("Failed to stop process".to_string()).into()),
        Err(e) => Err(ProcessManagerError::CommandExecutionFailed(e.to_string()).into()),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list_processes)
            .service(start_process)
            .service(stop_process)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
