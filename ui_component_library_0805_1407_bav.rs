use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use serde_json::json;

// 定义用户界面组件的数据结构
#[derive(Serialize)]
struct UIComponent {
    id: String,
    type_: String,
    props: serde_json::Value,
}

// 定义错误类型
#[derive(Debug)]
enum Error {
    NotFound,
    InvalidInput,
}

// 实现错误类型为Responder，以便可以直接返回错误
impl Responder for Error {
    fn respond_to(self, _: &web::HttpRequest) -> HttpResponse {
        match self {
            Error::NotFound => HttpResponse::NotFound().json(json!({"error": "Component not found"})),
            Error::InvalidInput => HttpResponse::BadRequest().json(json!({"error": "Invalid input"})),
        }
    }
}

// UI组件库服务
async fn get_component(params: web::Path<(String,)>) -> Result<HttpResponse, Error> {
    let component_id = params.into_inner();
    // 模拟组件数据，实际应用中应从数据库或其他存储中获取
    let component = UIComponent {
        id: component_id,
        type_: "button".to_string(),
        props: json!({"text": "Click me"}),
    };

    // 返回组件数据
    Ok(HttpResponse::Ok().json(component))
}

// 启动HTTP服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/component/:id", web::get().to(get_component))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 文档注释
/// 用户界面组件库
///
/// 这个库提供了一个简单的HTTP接口来检索用户界面组件。
///
/// 组件由其ID检索，并返回JSON格式的数据结构。
///
/// # Examples
///
/// 使用curl命令检索组件：
///
/// ```bash
/// curl http://127.0.0.1:8080/component/button
/// ```
