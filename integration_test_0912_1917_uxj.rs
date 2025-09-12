// integration_test.rs
// 这是一个使用RUST和ACTIX框架的集成测试工具。
use actix_web::{web, App, HttpServer, Responder};
use actix_web::test;

// 定义一个简单的响应结构体
struct MyResponse;

// 实现Responder特性，返回一个简单的响应
impl Responder for MyResponse {
    type Error = std::convert::Infallible;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok().body("Hello, Actix!")
    }
}

// 定义一个简单的路由处理器
async fn index() -> impl Responder {
    MyResponse
}

#[actix_web::test]
async fn test_index() {
    // 创建一个测试服务器
    let app = test::init_service(App::new()
        .service(web::resource("/").to(index)))
        .await;
    
    // 发送一个GET请求到服务器
    let req = test::TestRequest::with_uri("/").to_request();
    let resp = app.call(req).await.unwrap();
    
    // 检查响应状态和内容
    assert!(resp.status().is_success());
    assert_eq!(resp.body().as_str(), Some("Hello, Actix!"));
}
