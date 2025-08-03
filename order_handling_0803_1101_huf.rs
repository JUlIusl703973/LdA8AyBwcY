// order_handling.rs
// 使用Actix框架实现订单处理流程

use actix::{Actor, Context, AsyncContext, StreamHandler, Handler};
use actix::fut::{ok, err, ready, FnBoxFuture};
use actix::prelude::*;
use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::time::Duration;

// 定义订单模型
#[derive(Deserialize, Debug)]
struct Order {
    product_id: u32,
    quantity: u32,
    customer_id: u32,
}

// 订单处理结果
enum OrderResult {
    Success,
    Error(String),
}

// 订单处理消息
enum OrderMessage {
    PlaceOrder(Order),
    ProcessOrder(Order),
    FinalizeOrder(Order),
}

// 订单处理者
struct OrderHandler;

impl Actor for OrderHandler {
    type Context = Context<Self>;
}

impl StreamHandler<Result<web::Json<Order>, actix_web::error::JsonPayloadError>> for OrderHandler {
    fn handle(&mut self, msg: Result<web::Json<Order>, actix_web::error::JsonPayloadError>, ctx: &mut Context<Self>) {
        let order = match msg {
            Ok(web::Json(order)) => order.into_inner(),
            Err(_) => {
                ctx.text(actix_web::HttpResponse::BadRequest().finish());
                return;
            }
        };
        self.handle_process_order(order, ctx);
    }
}

impl OrderHandler {
    fn handle_process_order(&self, order: Order, ctx: &mut Context<Self>) -> impl ActorFuture<Actor = Self, Output = OrderResult> {
        let fut = async move {
            // 模拟订单处理逻辑
            if order.quantity > 0 {
                // 订单有效，继续处理
                let result = self.handle_finalize_order(order).await;
                Ok(result)
            } else {
                // 订单无效，返回错误
                Err(