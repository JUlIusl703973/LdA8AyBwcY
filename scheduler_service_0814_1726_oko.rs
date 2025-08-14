use actix::prelude::*;
use actix::System;
use std::time::Duration;
use tokio::time::{interval, Interval};
use std::sync::Arc;
use std::collections::HashMap;

/// 定时任务调度器
pub struct Scheduler {
    tasks: Arc<HashMap<String, Interval>>,
}

impl Scheduler {
    /// 创建一个新的调度器
    pub fn new() -> Self {
        Scheduler {
            tasks: Arc::new(HashMap::new()),
        }
    }

    /// 添加一个定时任务
    pub fn add_task(&self, task_name: String, interval_ms: u64) {
        let interval = interval(Duration::from_millis(interval_ms));
        self.tasks.insert(task_name, interval);
    }

    /// 启动所有定时任务
    pub async fn start_tasks(&self) {
        for (task_name, interval) in self.tasks.iter() {
            let task_name = task_name.clone();
            let interval = interval.clone();
            let task = async move {
                loop {
                    interval.tick().await;
                    println!("Task {} executed", task_name);
                }
            };
            actix::spawn(task);
        }
    }
}

#[actix::main]
async fn main() {
    // 初始化系统
    System::new().block_on(async {
        // 创建调度器
        let scheduler = Scheduler::new();

        // 添加定时任务
        scheduler.add_task("task1".to_string(), 1000);
        scheduler.add_task("task2".to_string(), 2000);

        // 启动定时任务
        scheduler.start_tasks().await;
    });
}