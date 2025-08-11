use actix_web::{web, App, HttpServer, Responder, get, post, HttpRequest, HttpResponse};
use sha2::{Sha256, Digest};
use hex;
use std::io::{self, Write};

/// 计算字符串的SHA-256哈希值
/// 函数接受一个字符串参数，返回其SHA-256哈希值的十六进制表示
fn calculate_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

/// 处理GET请求的路由，返回简单的欢迎消息
/// 用于测试和确认服务已启动
#[get("/")]
async fn index() -> impl Responder {
    "Welcome to the Hash Calculator Service!"
}

/// 处理POST请求的路由，计算并返回请求体的SHA-256哈希值
/// 需要在请求体中包含要哈希的字符串
#[post("/hash")]
async fn hash_data(req: HttpRequest) -> impl Responder {
    let body = req.body();
    if body.is_empty() {
        return HttpResponse::BadRequest().json("Request body is empty");
    }

    let input = match web::block(|| body.limit(1024 * 1024)).await {
        Ok(body) => String::from_utf8(body.to_vec()).map_err(|err| {
            eprintln!("Error parsing body: {}", err);
            HttpResponse::BadRequest().json("Failed to parse request body")
        }),
        Err(_) => return HttpResponse::BadRequest().json("Failed to read request body"),
    };

    match input {
        Ok(text) => HttpResponse::Ok().json(calculate_sha256(&text)),
        Err(resp) => resp,
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hash_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 代码结束