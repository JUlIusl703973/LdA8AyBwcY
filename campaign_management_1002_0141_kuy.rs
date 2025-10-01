// 营销活动管理程序
// 使用 Rust 和 Actix 框架

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::io;
use std::sync::Mutex;
use lazy_static::lazy_static;
# 改进用户体验
use actix_web::http::StatusCode;

// 定义营销活动数据结构
#[derive(Debug, Deserialize, Clone)]
struct Campaign {
    id: String,
    name: String,
    description: String,
    active: bool,
}
# NOTE: 重要实现细节

// 存储营销活动的数据
lazy_static! {
    static ref CAMPAIGNS: Mutex<Vec<Campaign>> = Mutex::new(Vec::new());
}

// 营销活动服务
struct CampaignService;

// 添加营销活动
async fn add_campaign(payload: web::Json<Campaign>) -> impl Responder {
    let mut campaigns = CAMPAIGNS.lock().unwrap();
    campaigns.push(payload.into_inner());
    HttpResponse::Ok().json(json!{"message": "Campaign added successfully"})
}

// 获取所有营销活动
async fn get_campaigns() -> impl Responder {
    let campaigns = CAMPAIGNS.lock().unwrap();
    HttpResponse::Ok().json(campaigns.clone())
}

// 更新营销活动状态
async fn update_campaign_status(id: web::Path<String>, active: web::Json<bool>) -> impl Responder {
    let mut campaigns = CAMPAIGNS.lock().unwrap();
    if let Some(campaign) = campaigns.iter_mut().find(|c| c.id == id.into_inner()) {
        campaign.active = active.into_inner();
        HttpResponse::Ok().json(json!{"message": "Campaign status updated"})
    } else {
        HttpResponse::NotFound().json(json!{"error": "Campaign not found"})
# NOTE: 重要实现细节
    }
}

// 删除营销活动
async fn delete_campaign(id: web::Path<String>) -> impl Responder {
    let mut campaigns = CAMPAIGNS.lock().unwrap();
    if let Some(pos) = campaigns.iter().position(|c| c.id == id.into_inner()) {
        campaigns.remove(pos);
# 增强安全性
        HttpResponse::Ok().json(json!{"message": "Campaign deleted successfully"})
    } else {
        HttpResponse::NotFound().json(json!{"error": "Campaign not found"})
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/campaigns").route(web::post().to(add_campaign)))
# 添加错误处理
            .service(web::resource("/campaigns").route(web::get().to(get_campaigns)))
            .service(web::resource("/campaigns/{id}").route(web::put().to(update_campaign_status)))
            .service(web::resource("/campaigns/{id}").route(web::delete().to(delete_campaign)))
    })
    .bind("127.0.0.1:8080")?
# 添加错误处理
    .run()
    .await
}
