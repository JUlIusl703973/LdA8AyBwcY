use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

// 定义一个简单的测试报告结构体，用于序列化数据
#[derive(Serialize)]
struct TestReport {
    test_name: String,
    start_time: SystemTime,
    end_time: SystemTime,
    status: String,
    details: Option<String>,
}

// 定义一个处理生成测试报告的函数
#[get("/report")]
async fn generate_test_report() -> impl Responder {
    // 定义测试报告的基本信息
    let report = TestReport {
        test_name: "Integration Test".to_string(),
        start_time: SystemTime::now(),
        end_time: SystemTime::now(), // 这里假设测试立即结束，实际应用中需要更新为实际结束时间
        status: "Passed".to_string(),
        details: Some("Test executed successfully".to_string()),
    };

    // 将报告信息序列化为JSON字符串
    let report_json = serde_json::to_string(&report).unwrap_or_else(|_| "{"error":"Failed to generate report"}".to_string());

    // 将报告写入文件
    let file_name = format!("test_report_{}.md", report.test_name.replace(" ", "_"));
    let path = Path::new(&file_name);
    let mut file = File::create(&path).expect("Unable to create test report file");
    writeln!(file, "{}", report_json).expect("Unable to write to test report file");

    // 返回测试报告文件路径和状态
    HttpResponse::Ok().json({
        "message": "Test report generated successfully",
        "file_path": file_name,
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .service(generate_test_report)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
