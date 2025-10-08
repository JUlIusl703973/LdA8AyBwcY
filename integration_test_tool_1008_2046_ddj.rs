use actix_web::{test, App, HttpServer, Responder};

// 定义一个简单的响应结构
struct MyResponse;

// 实现Responder trait，定义返回的数据格式
impl Responder for MyResponse {
    type Error = std::io::Error;
    type Future = std::future::Ready<Result<actix_web::HttpResponse, Self::Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
        // 返回一个简单的HTTP响应
        std::future::ready(Ok(actix_web::HttpResponse::Ok().finish()))
    }
}

// 创建一个简单的路由处理器
async fn index() -> impl Responder {
    MyResponse
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建HTTP服务器并监听127.0.0.1:8080
    HttpServer::new(|| {
        // 定义路由
        App::new()
            .route("/", actix_web::web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 集成测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_index() {
        // 创建测试服务器
        let app = test::init_service(App::new().route("/", actix_web::web::get().to(index))).await;

        // 发送GET请求到根路径
        let req = test::TestRequest::with_uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
