use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, HttpMessage, Error, Result};
use std::process::Command;
use std::io;
# 添加错误处理
use std::collections::HashMap;
use serde::Serialize;
use serde_json::json;

// 定义进程信息结构体
#[derive(Serialize)]
struct ProcessInfo {
# 增强安全性
    name: String,
    status: String,
}
# TODO: 优化性能

// 主应用结构体
struct ProcessManager;

impl ProcessManager {
    // 启动新进程的函数
    fn start_process(&self, command: &str) -> Result<HttpResponse, Error> {
# 改进用户体验
        let mut child = Command::new("/bin/sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let output = child.wait_with_output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let status = match child.status()?.code() {
            Some(code) => format!("Exited with code {}", code),
            None => "Killed by signal".to_string(),
        };
        Ok(HttpResponse::Ok().json(json!({
            "stdout": stdout,
            "stderr": stderr,
# 扩展功能模块
            "status": status,
        })))
    }

    // 获取当前所有进程信息
    fn list_processes(&self) -> Result<HttpResponse, Error> {
# 添加错误处理
        let mut processes = HashMap::new();
# TODO: 优化性能
        for pid in 1.. {
            let pid = Pid::from_raw_nonzero(pid as u32);
            match Process::new(pid) {
                Ok(process) => {
                    let name = process.name().unwrap_or_else(|_| "Unknown".to_string());
                    let status = process.status().map_or("Unknown".to_string(), |status| match status {
                        ProcessStatus::Running => "Running".to_string(),
                        ProcessStatus::Stopped => "Stopped".to_string(),
                        ProcessStatus::Zombie => "Zombie".to_string(),
                        _ => "Unknown".to_string(),
                    });
                    processes.insert(name.clone(), ProcessInfo { name, status });
                },
                Err(_) => break, // 如果无法获取进程信息，则终止循环
            }
        }
        Ok(HttpResponse::Ok().json(json!({
            "processes": processes,
        })))
    }
# 改进用户体验
}
# 优化算法效率

// 配置Actix Web服务器
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Process Manager!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/start", web::post().to(ProcessManager::start_process))
            .route("/list", web::get().to(ProcessManager::list_processes))
            .default_service(
                web::route().to(index),
# 扩展功能模块
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
# 增强安全性
