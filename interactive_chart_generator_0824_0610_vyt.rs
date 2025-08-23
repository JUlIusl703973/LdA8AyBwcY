use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
# NOTE: 重要实现细节
use std::sync::Mutex;
use once_cell::sync::Lazy;
use plotters::prelude::*;

// 全局数据存储
static DATA: Lazy<Mutex<HashMap<String, Vec<(f64, f64)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
# TODO: 优化性能

#[derive(Serialize, Deserialize)]
struct ChartData {
    x: Vec<f64>,
    y: Vec<f64>,
}

#[get("/add_data/{chart_name}")]
async fn add_data(chart_name: web::Path<String>, body: web::Json<ChartData>) -> impl Responder {
    let mut data = DATA.lock().unwrap();
    let chart_data = data.entry(chart_name.into_inner()).or_insert_with(Vec::new);
# 改进用户体验
    for (x, y) in body.into_inner().x.iter().zip(body.into_inner().y.iter()) {
        chart_data.push((*x, *y));
# TODO: 优化性能
    }
# 扩展功能模块
    HttpResponse::Ok().json({"message": "Data added successfully"})
}

#[get("/generate_chart/{chart_name}")]
async fn generate_chart(chart_name: web::Path<String>) -> impl Responder {
    let data = DATA.lock().unwrap();
# 改进用户体验
    if let Some(chart_data) = data.get(&chart_name.into_inner()) {
        let drawing_area = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
# NOTE: 重要实现细节
        drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("Interactive Chart", ("sans-serif", 50))
            .build_cartesian_2d(0f64..100f64, 0f64..100f64).unwrap();

        chart
            .configure_mesh()
# FIXME: 处理边界情况
            .draw()
            .unwrap();

        for (x, y) in chart_data.iter() {
            chart.draw_series(PointSeries::of_element(
                [(*x, *y).into()],
                &BLUE.mix(0.5),
                &|coord, size, color| {
                    PrimitiveStyle::with_fill(&color).marker(&ShapeStyle::circle(2, color))
# 增强安全性
                },
            ))
# 改进用户体验
            .unwrap();
        }

        drawing_area.present().unwrap();
        HttpResponse::Ok().content_type("image/png").body(drawing_area.to_image())
    } else {
# FIXME: 处理边界情况
        HttpResponse::NotFound().json({"error": "Chart not found"})
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add_data)
            .service(generate_chart)
    })
    .bind("127.0.0.1:8080")?
# NOTE: 重要实现细节
    .run()
    .await
}
