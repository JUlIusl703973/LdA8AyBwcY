use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use reqwest;
use std::error::Error;
use url::Url;
use anyhow::Result;
use serde::Serialize;
# 增强安全性
use serde::de::DeserializeOwned;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::{ErrorInternalServerError, ErrorBadRequest, ErrorUnauthorized};
use actix_web::middleware::Logger;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::Error;
# FIXME: 处理边界情况

// 定义一个结构体来存储网页内容
#[derive(Serialize, Deserialize, Debug)]
struct WebContent {
# 改进用户体验
    status_code: u16,
    content_type: String,
    content: String,
}

// 异步函数，抓取网页内容
# 优化算法效率
async fn fetch_web_content(req: ServiceRequest, url: web::Data<Url>) -> Result<ServiceResponse, Error> {
    let client = reqwest::Client::new();
    let response = client.get(url.as_str()).send().await?;
    let status_code = response.status().as_u16();
    let content_type = response.headers().get(CONTENT_TYPE).unwrap_or(&actix_web::http::header::HeaderValue::from_static("text/plain")).as_bytes().to_vec();
    let content = response.text().await?;
# 改进用户体验

    Ok(ServiceResponse::new(req.into_parts().0,
        HttpResponse::Ok().content_type(&content_type).body(content)))
}

// Actix Web 服务配置
# 优化算法效率
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
# 增强安全性
        App::new()
            .wrap(Logger::default())
            // 处理错误页面
# NOTE: 重要实现细节
            .default_service(
                web::resource("/{tail:.*}").route(web::get().to(handle_request))
            )
# 扩展功能模块
            // 添加自定义中间件
            .wrap(Logger::default())
# 增强安全性
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 处理请求的函数
async fn handle_request(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    let url_str = req.match_info().get("tail").unwrap_or("");
    let url = Url::parse(url_str).map_err(|e| ErrorBadRequest(e.to_string()))?;
    
    // 调用异步函数抓取网页内容
# 添加错误处理
    let response = fetch_web_content(req, web::Data::new(url)).await;
    
    // 处理可能的错误
    match response {
        Ok(resp) => Ok(resp),
        Err(e) => Err(match e.downcast_ref::<reqwest::Error>() {
            Some(_) => ErrorInternalServerError("Failed to fetch the URL"),
            None => e,
# FIXME: 处理边界情况
        }),
    }
}

// 错误处理
fn error_handler(err: Error, _req: &ServiceRequest) -> ErrorHandlerResponse {
    match err {
        ErrorBadRequest(_) => ErrorHandlerResponse::Response(HttpResponse::BadRequest().finish()),
        ErrorUnauthorized(_) => ErrorHandlerResponse::Response(HttpResponse::Unauthorized().finish()),
# FIXME: 处理边界情况
        _ => ErrorHandlerResponse::Response(HttpResponse::InternalServerError().finish()),
    }
}
