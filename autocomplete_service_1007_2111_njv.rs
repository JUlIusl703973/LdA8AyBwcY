use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use futures::future::{self, Future};
use futures::FutureExt;

/// 搜索自动补全服务
#[get("/auto_complete/{query}")]
async fn auto_complete(query: web::Path<String>) -> impl Responder {
    let query = query.into_inner();
    // 这里只是一个示例，实际的搜索逻辑需要根据需求实现
    let suggestions = vec![
        "apple",
        "banana",
        "orange",
        "grape",
        "mango",
    ]
    .into_iter()
    .filter(|suggestion| suggestion.contains(&query))
    .take(5)
    .collect::<Vec<&str>>();

    // 将搜索建议转换为JSON响应
    let response = serde_json::json!({
# 增强安全性
        "query": query,
        "suggestions": suggestions
    });

    HttpResponse::Ok().content_type("application/json").body(response.to_string())
}

#[actix_web::main]
# FIXME: 处理边界情况
async fn main() -> std::io::Result<()> {
    // 在本地启动服务器
# TODO: 优化性能
    HttpServer::new(|| {
        App::new()
            // 注册自动补全端点
            .service(auto_complete)
    })
    // 绑定端口并启动服务器
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
# 优化算法效率
