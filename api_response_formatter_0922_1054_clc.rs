use actix_web::{get, HttpResponse, Responder, web};

// 定义一个结构体来封装API响应的数据
#[derive(Debug, Clone)]
struct ApiResponse {
    code: i32,
    message: String,
    data: Option<web::Json>,
}

// 定义一个函数来格式化API响应
fn format_response(code: i32, message: &str, data: Option<web::Json>) -> ApiResponse {
    ApiResponse {
        code,
        message: message.to_string(),
        data,
    }
}

// 定义一个函数来处理API响应并返回HttpResponse
async fn handle_response(response: ApiResponse) -> impl Responder {
    match serde_json::to_string(&response) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 定义一个GET路由来测试API响应格式化工具
#[get("/test")]
async fn test_handler() -> impl Responder {
    let response = format_response(200, "Success", Some(serde_json::json!({"key": "value"})));
    handle_response(response).await
}

// 定义Actix服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 定义Actix服务器的配置
    let server = actix_web::HttpServer::new(|| {
        // 定义App的配置
        actix_web::App::new()
            .route("/test", web::get().to(test_handler))
    })
    .bind("127.0.0.1:8080")?
    .run();
    
    // 运行服务器
    server.await
}
