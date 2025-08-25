use actix_web::{web, App, HttpServer, Responder, get};
use std::sync::Arc;
use sys_info::{loadavg, CpuExt, System, SystemExt};
use tokio::sync::Mutex;

/// 系统信息封装结构体，使用Arc和Mutex实现线程安全
struct SystemInfo {
    system: System,
}

impl SystemInfo {
    /// 初始化系统信息
    fn new() -> Self {
        SystemInfo {
            system: System::new(),
        }
    }

    /// 获取CPU使用率
    async fn cpu_usage(&self) -> f32 {
        let cpu_usage = self.system.cpu().await.unwrap_or_default().cpu_usage();
        cpu_usage
    }

    /// 获取系统负载平均值
    async fn load_average(&self) -> Vec<f64> {
        let loadavg = loadavg().unwrap_or_default();
        loadavg
    }
}

/// 系统性能监控服务
/// 使用Actix Web提供HTTP接口
struct SystemMonitorService {
    info: Arc<Mutex<SystemInfo>>,
}

impl SystemMonitorService {
    /// 初始化系统监控服务
    fn new() -> Self {
        SystemMonitorService {
            info: Arc::new(Mutex::new(SystemInfo::new())),
        }
    }
}

/// 获取CPU使用率
/// 路由：/monitor/cpu
#[get(