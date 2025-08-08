use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};

// 定义一个结构体来处理随机数生成
struct RandomNumberGenerator;

// 实现随机数生成的函数
#[get("/random/{count}")]
async fn generate_random_numbers(count: web::Path<u32>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u32> = (0..count).map(|_| rng.gen()).collect();

    // 错误处理：如果count无效，则返回错误信息
    if count == 0 {
        return HttpResponse::BadRequest().body("Count must be greater than 0");
    }

    HttpResponse::Ok().json(numbers)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            // 将路由加入到应用程序中
            .service(generate_random_numbers)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 引入rand库用于生成随机数
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web};
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_generate_random_numbers() {
        let app = test::init_service(App::new().service(generate_random_numbers)).await;
        let req = test::TestRequest::with_uri("/random/10").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
