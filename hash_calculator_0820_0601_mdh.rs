use actix_web::{get, HttpResponse, Responder};
use std::hash::{Hash, Hasher};
use twox_hash::XxHash64; // 使用twox_hash库的XxHash64哈希算法

/// 哈希值计算工具
///
/// 这个工具使用Rust的twox_hash库来计算给定字符串的哈希值。
#[get("/hash/{input}")]
async fn hash_input(input: String) -> impl Responder {
    // 使用XxHash64哈希算法计算字符串的哈希值
    let mut hasher = XxHash64::default();
    let hash_value = input.hash(&mut hasher);

    // 将哈希值转换为十六进制字符串
    let hash_hex = format!("{:016x}", hash_value);

    // 返回哈希值的响应
    HttpResponse::Ok().body(hash_hex)
}

/// 主函数，设置Actix服务器并启动监听
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动Actix Web服务器
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            // 注册路由，将输入的字符串作为参数
            .route("/hash/:input", get(hash_input))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
