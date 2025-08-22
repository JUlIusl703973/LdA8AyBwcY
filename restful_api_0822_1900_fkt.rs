use actix_web::{
    get,
    HttpResponse,
    Responder,
    web,
    App,
# 优化算法效率
    HttpServer,
};

// 定义一个结构体来表示我们的数据模型
struct User {
    id: i32,
# NOTE: 重要实现细节
    name: String,
}
# 优化算法效率

// 定义一个错误类型
#[derive(Debug)]
enum AppError {
    NotFound,
    InternalError,
}

// 实现Responder特性，以便AppError可以作为响应返回
impl Responder for AppError {
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        match self {
# 添加错误处理
            AppError::NotFound => HttpResponse::NotFound().finish(),
# 扩展功能模块
            AppError::InternalError => HttpResponse::InternalServerError().finish(),
        }
    }
}

// GET路由处理函数，返回一个User结构体的实例
#[get("/user/{id}")]
# TODO: 优化性能
async fn user_handler(id: i32) -> Result<&'static str, AppError> {
    // 模拟数据库操作，根据id查找用户
    let user = match id {
        1 => Some(User { id: 1, name: "John Doe".to_string() }),
# 增强安全性
        _ => None,
    };

    // 如果找到用户，返回用户的名称，否则返回错误
    match user {
# 增强安全性
        Some(_) => Ok("User found"),
        None => Err(AppError::NotFound),
    }
}

// 启动服务器的主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器并监听8080端口
    HttpServer::new(|| {
        App::new()
# 优化算法效率
            .route("/user/{id}", web::get().to(user_handler))
    })
    .bind("127.0.0.1:8080")?
# 添加错误处理
    .run()
    .await
}
