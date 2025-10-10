// datetime_selector.rs
// 使用Actix框架实现的日期时间选择器。

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::json;

// 定义一个请求参数结构体，用于解析客户端传递的参数。
#[derive(Deserialize)]
pub struct DateTimeParams {
    date: String,
    time: String,
}

// 定义一个响应结构体，用于构建JSON响应。
pub struct DateTimeResponse {
    date_time: DateTime<Local>,
}

// 实现Responder trait，使DateTimeResponse可以作为响应返回。
impl Responder for DateTimeResponse {
    type Error = std::convert::Infallible;

    fn respond_to(self, _req: &HttpRequest) -> Result<HttpResponse, Self::Error> {
        let serialized = json!({
            "date_time": self.date_time.format("%Y-%m-%d %H:%M:%S")
        }).to_string();
        Ok(HttpResponse::Ok().content_type("application/json").body(serialized))
    }
}

// 日期时间选择器的路由处理器。
#[get("/datetime/{date}/{time}")]
async fn datetime_selector(params: web::Path<DateTimeParams>) -> impl Responder {
    // 尝试从参数中解析日期和时间。
    let date = DateTime::parse_from_str(&params.date, "%Y-%m-%d")
        .map_err(|_| HttpResponse::BadRequest().json(json!({"error": "Invalid date format"})))?;
    let time = DateTime::parse_from_str(&params.time, "%H:%M:%S")
        .map_err(|_| HttpResponse::BadRequest().json(json!({"error": "Invalid time format"})))?;
        
    // 将日期和时间合并为一个DateTime对象。
    let datetime = date.and_time(time.time());

    // 返回日期时间选择器的响应。
    DateTimeResponse { date_time: datetime.with_timezone(&Local) }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器。
    HttpServer::new(|| {
        App::new()
            .service(datetime_selector)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
