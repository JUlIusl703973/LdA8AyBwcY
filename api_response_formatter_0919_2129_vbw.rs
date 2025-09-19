use actix_web::{get, HttpResponse, Responder, web};
use serde::Serialize;
use serde_json::json;

/// API响应格式化工具
/// 这个结构体封装了API响应的格式化逻辑
#[derive(Serialize)]
struct ApiResponse<T> {
    /// 状态码
    #[serde(rename = "code")]
    status_code: u16,
    /// 消息
    #[serde(rename = "message")]
    message: String,
    /// 数据
    #[serde(rename = "data")]
    data: T,
}

/// API响应格式化工具的实现
impl ApiResponse<()> {
    /// 创建一个成功的API响应
    fn success(message: &str) -> ApiResponse<()> {
        ApiResponse {
            status_code: 200,
            message: message.to_string(),
            data: (),
        }
    }

    /// 创建一个错误的API响应
    fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            status_code: 500,
            message: message.to_string(),
            data: (),
        }
    }
}

/// API控制器
/// 这个结构体封装了API请求的处理逻辑
struct ApiController;

impl ApiController {
    /// 处理GET请求，返回格式化的API响应
    #[get("/")]
    async fn index() -> impl Responder {
        let response = ApiResponse::success("Welcome to the API Response Formatter!");
        HttpResponse::Ok().json(response)
    }

    /// 处理GET请求，模拟错误情况
    #[get("/error")]
    async fn error() -> impl Responder {
        let response = ApiResponse::error("An error occurred.");
        HttpResponse::InternalServerError().json(response)
    }
}

/// 程序入口
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(ApiController::index)
            .service(ApiController::error)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
