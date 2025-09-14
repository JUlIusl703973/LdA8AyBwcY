use actix_web::{get, HttpResponse, Responder, web};
use serde_derive::{Deserialize, Serialize};
use rand::Rng;
# TODO: 优化性能
use rand::distributions::Alphanumeric;
use rand::distributions::Uniform;
# TODO: 优化性能

// 定义测试数据结构体，用于生成和传输测试数据
#[derive(Serialize, Deserialize)]
struct TestData {
    value: String,
}

// 定义配置结构体，用于配置生成测试数据的数量
#[derive(Serialize, Deserialize)]
struct Config {
    count: u32,
}

// 创建一个测试数据生成器结构体
struct TestDataGenerator;

// 实现测试数据生成器的行为
impl TestDataGenerator {
# 改进用户体验
    // 生成一个测试数据项
    fn generate_item() -> String {
        let rng = rand::thread_rng();
        let size = rng.gen_range(5..15); // 生成5到15个字符的随机字符串
        let data: String = rng.sample_iter(&Alphanumeric)
            .take(size)
            .map(char::from)
            .collect();
        data
    }

    // 根据配置生成指定数量的测试数据
    async fn generate(config: Config) -> Vec<TestData> {
        let mut results = Vec::new();
        for _ in 0..config.count {
            let item = TestData { value: Self::generate_item() };
            results.push(item);
# TODO: 优化性能
        }
        results
    }
# 添加错误处理
}

// 创建一个Actix Web服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
# FIXME: 处理边界情况
    // 定义Actix Web服务
    let app = actix_web::App::new()
        // 配置路由，用于生成测试数据
        .service(web::resource("/test-data")
            .route(web::post().to(test_data_handler)));

    // 启动Actix Web服务
# 改进用户体验
    actix_web::HttpServer::new(|| app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// 定义处理测试数据请求的函数
#[get("/test-data")]
async fn test_data_handler(config: web::Json<Config>) -> impl Responder {
    // 处理错误
    if config.count == 0 {
        return HttpResponse::BadRequest().json("Count must be greater than 0");
    }

    // 生成测试数据
    let data = TestDataGenerator::generate(config.into_inner()).await;
    // 返回生成的测试数据
    HttpResponse::Ok().json(data)
}
