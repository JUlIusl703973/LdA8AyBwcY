use actix::prelude::*;
use actix_web::web;
use tokio::time::interval;
use std::time::Duration;

/// SchedulerService 定时任务调度器
# 扩展功能模块
pub struct SchedulerService;
# TODO: 优化性能

impl Actor for SchedulerService {
    type Context = Context<Self>;
}

/// SchedulerService 消息，用于定时触发任务
#[derive(Message)]
#[rtype(result = "()