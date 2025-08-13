use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// 定义一个结构体来封装数学计算工具集
struct MathToolkit;

// 实现MathToolkit结构体的方法
impl MathToolkit {
    // 加法
    #[get("/add/{x}/{y}")]
    async fn add(x: f64, y: f64) -> impl Responder {
        match x.checked_add(y) {
            Some(sum) => HttpResponse::Ok().json(sum),
            None => HttpResponse::BadRequest().body("Overflow or invalid input."),
        }
    }

    // 减法
    #[get("/sub/{x}/{y}")]
    async fn sub(x: f64, y: f64) -> impl Responder {
        match x.checked_sub(y) {
            Some(diff) => HttpResponse::Ok().json(diff),
            None => HttpResponse::BadRequest().body("Overflow or invalid input."),
        }
    }

    // 乘法
    #[get("/mul/{x}/{y}")]
    async fn mul(x: f64, y: f64) -> impl Responder {
        match x.checked_mul(y) {
            Some(prod) => HttpResponse::Ok().json(prod),
            None => HttpResponse::BadRequest().body("Overflow or invalid input."),
        }
    }

    // 除法
    #[get("/div/{x}/{y}")]
    async fn div(x: f64, y: f64) -> impl Responder {
        if y == 0.0 {
            HttpResponse::BadRequest().body("Cannot divide by zero.")
        } else {
            match x.checked_div(y) {
                Some(quot) => HttpResponse::Ok().json(quot),
                None => HttpResponse::BadRequest().body("Overflow or invalid input."),
            }
        }
    }
}

// 启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 注册数学计算工具集的方法
            .route("/add/{x}/{y}", web::get().to(MathToolkit::add))
            .route("/sub/{x}/{y}", web::get().to(MathToolkit::sub))
            .route("/mul/{x}/{y}", web::get().to(MathToolkit::mul))
            .route("/div/{x}/{y}", web::get().to(MathToolkit::div))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
