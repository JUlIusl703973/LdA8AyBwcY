use actix_web::{
    web, App, HttpResponse, HttpServer, Responder,
};

// 定义一个简单的结构体，表示用户信息
#[derive(Debug, serde::Deserialize)]
struct UserInfo {
    username: String,
# 扩展功能模块
    email: String,
}

// 创建一个异步函数处理GET请求，返回JSON格式的用户信息
# 增强安全性
async fn get_user_info(info: web::Json<UserInfo>) -> impl Responder {
    // 简单的错误处理
    if info.username.is_empty() {
        return HttpResponse::BadRequest().json("Username cannot be empty");
    }

    // 构造响应数据
    let response = serde_json::json!({
        "status": "success",
        "data": info,
    });

    // 返回成功响应
    HttpResponse::Ok().json(response)
}

// 入口函数，设置服务和路由
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
# 增强安全性
        App::new()
            // 添加路由，路径为"/user"，处理方法为POST
            .route("/user", web::post().to(get_user_info))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 为了测试，可以添加单元测试
# 改进用户体验
#[cfg(test)]
mod tests {
    use super::*;
# NOTE: 重要实现细节
    use actix_web::{test, web};
    use serde_json::json;

    #[actix_web::test]
    async fn test_get_user_info() {
# FIXME: 处理边界情况
        let app = test::init_service(App::new().route("/user", web::post().to(get_user_info))).await;
        let req = test::TestRequest::post()
            .uri("/user")
# 添加错误处理
            .set_json(&json!({
                "username": "testuser",
                "email": "test@example.com",
            }))
            .to_request();

        let resp = app.call(req).await;
        assert!(resp.status().is_success());
    }
# FIXME: 处理边界情况
}
