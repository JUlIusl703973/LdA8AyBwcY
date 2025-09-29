use actix::prelude::*;
use std::time::Duration;
use actix::System;

// 定义定时任务调度器结构体
struct ScheduledTaskScheduler;

// 实现 Actor 特质，将 ScheduledTaskScheduler 作为 Actor
impl Actor for ScheduledTaskScheduler {
    type Context = Context<Self>;
}

// 实现 Handler 来处理启动定时任务的消息
impl Handler<StartScheduledTask> for ScheduledTaskScheduler {
    type Result = ();

    fn handle(&mut self, _msg: StartScheduledTask, _ctx: &mut Self::Context) -> Self::Result {
        // 启动定时任务
        println!("Scheduled task started.");
        Self::schedule_task();
    }
}

// 定义启动定时任务的消息
#[derive(Message)]
#[rtype(result = "()