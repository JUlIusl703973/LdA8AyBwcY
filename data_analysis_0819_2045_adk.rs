use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// 定义统计数据结构
#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    value: f64,
    category: String,
}

// 定义统计分析请求结构
#[derive(Deserialize)]
struct AnalysisRequest {
    data: Vec<DataPoint>,
}

// 实现统计分析器
struct DataAnalysis;

impl DataAnalysis {
    // 计算平均值
    fn calculate_mean(&self, data: &[DataPoint]) -> f64 {
        data.iter().map(|dp| dp.value).sum::<f64>() / data.len() as f64
    }

    // 计算中位数
    fn calculate_median(&self, data: &[DataPoint]) -> f64 {
        let mut values: Vec<f64> = data.iter().map(|dp| dp.value).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if values.len() % 2 == 0 {
            (values[values.len() / 2 - 1] + values[values.len() / 2]) / 2.0
        } else {
            values[values.len() / 2]
        }
    }

    // 计算最大值
    fn calculate_max(&self, data: &[DataPoint]) -> f64 {
        *data.iter().max_by(|a, b| a.value.partial_cmp(&b.value).unwrap()).unwrap().value
    }

    // 计算最小值
    fn calculate_min(&self, data: &[DataPoint]) -> f64 {
        *data.iter().min_by(|a, b| a.value.partial_cmp(&b.value).unwrap()).unwrap().value
    }
}

// 定义统计分析响应结构
#[derive(Serialize, Debug)]
struct AnalysisResponse {
    mean: f64,
    median: f64,
    max: f64,
    min: f64,
}

// 定义HTTP处理器
async fn analyze_data(data: web::Json<AnalysisRequest>, data_analysis: web::Data<DataAnalysis>) -> impl Responder {
    let data_points = &data.data;
    let mean = data_analysis.calculate_mean(data_points);
    let median = data_analysis.calculate_median(data_points);
    let max = data_analysis.calculate_max(data_points);
    let min = data_analysis.calculate_min(data_points);

    HttpResponse::Ok().json(AnalysisResponse { mean, median, max, min })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(DataAnalysis))
            .service(
                analyze_data
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
