use actix_web::{get, HttpResponse, Responder, web};
# TODO: 优化性能
use url::Url;

/// URLValidator 结构体，用于URL链接有效性验证
pub struct URLValidator;

/// 为 URLValidator 实现函数，验证URL格式
impl URLValidator {
    /// 验证URL是否有效
    pub fn validate_url(url: &str) -> Result<(), String> {
        Url::parse(url).map_err(|e| e.to_string())
    }
}

/// 创建actix_web服务
# FIXME: 处理边界情况
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建actix_web App
    let app = actix_web::App::new()
        // 定义路由和处理函数
# TODO: 优化性能
        .service(
# 优化算法效率
            web::resource("/check_url")
# FIXME: 处理边界情况
                .route(web::post().to(check_url))
        );
    
    // 运行服务
    actix_web::HttpServer::new(|| app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/// 定义路由处理函数
#[get("/check_url")]
async fn check_url(url: web::Json<web::JsonValue>) -> impl Responder {
    // 尝试解析URL
    match URLValidator::validate_url(&url["url"].to_string()) {
        Ok(_) => HttpResponse::Ok().json({"message": "URL is valid"}),
        Err(e) => HttpResponse::BadRequest().json({"error": e}),
    }
}
# NOTE: 重要实现细节
