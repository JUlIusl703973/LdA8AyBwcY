use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, put, delete, Responder as ActixResponder, error::ErrorBadRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;

// 定义用户模型
#[derive(Serialize, Deserialize, Debug)]
# NOTE: 重要实现细节
struct User {
    id: u32,
    username: String,
    permissions: Vec<String>,
}

// 定义权限管理系统结构体
struct PermissionManager;

// 实现权限管理系统的方法
impl PermissionManager {
    // 获取用户权限
    fn get_user_permissions(user_id: u32) -> Result<User, Error> {
        // 这里应该是数据库查询逻辑，返回User或错误
        // 为了演示，我们返回一个硬编码的用户示例
        Ok(User {
            id: user_id,
            username: 