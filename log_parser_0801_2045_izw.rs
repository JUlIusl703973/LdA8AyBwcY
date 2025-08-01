use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;
use serde::Serialize;
use serde_json::json;

// 定义日志记录的数据结构
#[derive(Debug, Serialize)]
struct LogRecord {
    timestamp: String,
    level: String,
    message: String,
}

// 定义错误处理结构体
#[derive(Debug)]
enum LogParseError {
    FileError(io::Error),
    ParseError,
}

// 实现错误处理的Display和Error trait
impl std::fmt::Display for LogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogParseError::FileError(e) => write!(f, "File error: {}", e),
            LogParseError::ParseError => write!(f, "Parse error"),
        }
    }
}

impl Error for LogParseError {}

// 解析日志文件
fn parse_log_file(file_path: &str) -> Result<Vec<LogRecord>, LogParseError> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // 假设日志格式为: [timestamp][level] message
        let parts: Vec<&str> = line.splitn(3, ']').collect();
        if parts.len() != 3 {
            return Err(LogParseError::ParseError);
        }
        let timestamp = parts[0].trim().to_string();
        let level = parts[1].trim().to_string();
        let message = parts[2].trim().to_string();
        records.push(LogRecord {
            timestamp,
            level,
            message,
        });
    }

    Ok(records)
}

// 定义日志解析的HTTP处理函数
async fn parse_log(req: HttpRequest) -> impl Responder {
    let file_path = req.query().get(