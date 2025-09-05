use actix_web::{get, HttpResponse, Responder, web};

// 定义一个结构体来表示我们的应用程序状态
struct RandomNumberGenerator;

// 实现随机数生成器的服务
impl RandomNumberGenerator {
    /// 生成一个随机数
    #[get("/random")]
    async fn generate_random(&self) -> impl Responder {
        // 使用rand库生成一个随机数
        let random_number = rand::random::<u32>();
        
        // 以JSON格式响应随机数
        HttpResponse::Ok().json(random_number)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志记录
    env_logger::init();

    // 启动服务器
    actix_web::HttpServer::new(|| {
        let app = actix_web::App::new()
            // 注册随机数生成器服务
            .service(web::resource("/random").to(RandomNumberGenerator::generate_random));
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 添加必要的依赖
// Cargo.toml
// [dependencies]
// actix-web = "4.0.0-beta.17"
// rand = "0.8.5"
// env_logger = "0.9.0"