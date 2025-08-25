use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error, get, post};
use serde::Deserialize;
use serde_json::json;
use std::sync::Mutex;
use std::sync::Arc;

// 模拟数据库存储
lazy_static::lazy_static! {
    static ref PAYMENTS: Mutex<Vec<Payment>> = Mutex::new(Vec::new());
}

#[derive(Debug, Clone)]
struct Payment {
    id: u32,
    amount: f64,
    status: PaymentStatus,
}

#[derive(Debug, Clone)]
enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

// 支付请求数据结构
#[derive(Deserialize)]
struct PaymentRequest {
    amount: f64,
}

// 支付响应数据结构
#[derive(Serialize)]
struct PaymentResponse {
    id: u32,
    amount: f64,
    status: String,
}

// 支付服务处理器
async fn create_payment(data: web::Json<PaymentRequest>) -> Result<impl Responder, Error> {
    let payment_id = {
        let payments = PAYMENTS.lock().unwrap();
        payments.len() as u32 + 1
    };

    let payment = Payment {
        id: payment_id,
        amount: data.amount,
        status: PaymentStatus::Pending,
    };

    let payments = PAYMENTS.lock().unwrap();
    payments.push(payment.clone());

    Ok(HttpResponse::Ok().json(PaymentResponse {
        id: payment.id,
        amount: payment.amount,
        status: payment.status.to_string(),
    }))
}

// 将支付状态转换为字符串
impl PaymentStatus {
    fn to_string(&self) -> String {
        match self {
            PaymentStatus::Pending => "Pending".to_string(),
            PaymentStatus::Completed => "Completed".to_string(),
            PaymentStatus::Failed => "Failed".to_string(),
        }
    }
}

// 程序入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/payment").route(post().to(create_payment)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 错误处理
impl From<PaymentStatus> for actix_web::Error {
    fn from(_: PaymentStatus) -> Self {
        actix_web::error::ErrorInternalServerError("Internal server error")
    }
}

// 错误处理
impl From<Payment> for actix_web::Error {
    fn from(_: Payment) -> Self {
        actix_web::error::ErrorBadRequest("Invalid payment data")
    }
}
