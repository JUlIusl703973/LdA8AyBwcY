use actix::prelude::*;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, error::ErrorBadRequest};
use serde::Deserialize;
use serde_json::json;

// 自定义错误类型
#[derive(Debug)]
pub enum TestError {
    InvalidInput(String),
}

// 实现错误转换为HttpResponse
impl Error for TestError {}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TestError::InvalidInput(ref err) => write!(f, "{}", err),
        },
    }
}

// 测试输入数据结构
#[derive(Deserialize)]
pub struct TestInput {
    input: String,
}

// 测试服务
pub struct TestService;

impl TestService {
    // 执行测试
    pub async fn run_test(&self, input: TestInput) -> impl Responder {
        match input.input.parse::<i32>() {
            Ok(_) => HttpResponse::Ok().json(json!({"result": "success"})),
            Err(_) => HttpResponse::BadRequest().json(json!({"error": "Invalid input"})),
        }
    }
}

// 配置路由
pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/test")
                .route(web::post().to(|input: web::Json<TestInput>| async move {
                    let test_service = TestService;
                    test_service.run_test(input.into_inner()).await
                })),
        )
        .service(
            web::resource("/error")
                .route(web::post().to(|input: web::Json<TestInput>| async move {
                    Err(TestError::InvalidInput("Custom error".to_string()))
                })),
        );
}

// 主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use serde_json::json;
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_test_service() {
        let app = test::init_service(App::new().configure(config_services)).await;
        let req = test::TestRequest::post()
            .uri("/test")
            .set_json(&json!({"input": "123"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_error_handling() {
        let app = test::init_service(App::new().configure(config_services)).await;
        let req = test::TestRequest::post()
            .uri("/error")
            .set_json(&json!({"input": "invalid"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::BAD_REQUEST);
    }
}