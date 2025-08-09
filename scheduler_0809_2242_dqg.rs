use actix::prelude::*;
# NOTE: 重要实现细节
use std::time::Duration;
use tokio::time::{interval, Interval};

/// Scheduler actor to handle periodic tasks.
# 扩展功能模块
/// This actor will run a task at a specified interval.
# FIXME: 处理边界情况
pub struct Scheduler;

impl Actor for Scheduler {
    type Context = Context<Self>;
}

/// Message to schedule a task.
pub struct ScheduleTask{
    pub interval: Duration,
    pub task: Box<dyn Fn() + Send + 'static>,
}

impl Message for ScheduleTask {
    type Result = Result<(), ()>;
}

impl Handler<ScheduleTask> for Scheduler {
# 优化算法效率
    fn handle(&mut self, msg: ScheduleTask, _: &mut Self::Context) -> Self::Result {
        println!("Scheduling task to run every {:?}