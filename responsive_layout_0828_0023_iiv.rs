use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as ActixResponder, Error as ActixError};
use serde::Deserialize;
use serde_json::json;

// 定义请求体结构，用于接收前端传递的数据
#[derive(Deserialize, Debug)]
struct LayoutSize {
    width: u32,
    height: u32,
}

// 定义响应结构
#[derive(Serialize, Debug)]
struct LayoutResponse {
    status: String,
    message: String,
    size: LayoutSize,
}

#[get("/layout/{width}/{height}")]
async fn responsive_layout(size: web::Path<(u32, u32)>) -> impl ActixResponder {
    // 解析路径参数
    let (width, height) = size.into_inner();

    // 构建响应体
    let response = LayoutResponse {
        status: "success".to_string(),
        message: "Responsive layout response".to_string(),
        size: LayoutSize { width, height },
    };

    // 将响应体序列化为JSON并返回
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> Result<(), ActixError> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            // 注册路由
            .service(responsive_layout)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    Ok(())
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web};
    use serde_json::json;

    #[actix_web::test]
    async fn test_responsive_layout() {
        // 测试响应式布局接口
        let mut app = test::init_service(App::new().service(responsive_layout)).await;

        // 发送请求
        let req = test::TestRequest::with_uri("/layout/1024/768").to_request();
        let resp = test::call_service(&mut app, req).await;

        // 验证响应状态码和内容
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let response: LayoutResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Responsive layout response");
        assert_eq!(response.size.width, 1024);
        assert_eq!(response.size.height, 768);
    }
}
