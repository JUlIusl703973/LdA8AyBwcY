use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;
use std::str::FromStr;

/// 定义一个结构体来处理JSON数据转换
struct JsonConverter;

impl JsonConverter {
    /// 将接收到的JSON字符串转换成另一种格式，这里简单示例将其转换为大写
    /// 
    /// # 参数
    /// - input: 接收到的JSON字符串
    /// # 返回
    /// - Result<String>: 转换后的JSON字符串或错误信息
    fn convert_to_upper(input: &str) -> Result<String, serde_json::Error> {
        let value: Value = serde_json::from_str(input)?;
        let result = serde_json::to_string(&value.as_str().unwrap().to_uppercase())?;
        Ok(result)
    }
}

/// 定义一个Actix Web的HTTP处理器，用于处理转换请求
async fn convert_json(data: web::Json<String>) -> impl Responder {
    match JsonConverter::convert_to_upper(&data.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/convert", web::post().to(convert_json))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
