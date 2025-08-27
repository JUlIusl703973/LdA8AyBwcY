use actix_web::{get, HttpResponse, Responder, web, App, HttpServer, Responder as ActixResponder};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// Main struct for our application
    #[derive(Debug)]
    struct InteractiveChartGenerator;

    #[derive(Serialize, Deserialize)]
    struct ChartData {
        title: String,
        labels: Vec<String>,
        values: Vec<f64>,
    }

    impl InteractiveChartGenerator {
        /// Function to generate a chart
        fn generate_chart(data: ChartData) -> String {
            // Here you would implement the chart generation logic
            // For demonstration purposes, we are just returning a string
            format!("Chart generated with title: {} and {} points
", data.title, data.values.len())
        }
    }

    /// Define the route for the application
    #[get("/generate")]
    async fn generate_chart_route(data: web::Json<ChartData>) -> impl ActixResponder {
        // Error handling if data is missing or invalid
        match InteractiveChartGenerator::generate_chart(data.into_inner()) {
            chart => HttpResponse::Ok().json(chart),
        }
    }

    /// Entry point of the application
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        // Start the HTTP server
        HttpServer::new(|| {
            App::new()
                .service(generate_chart_route)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }

    /// Function to handle errors and return appropriate responses
    fn handle_error(e: &str) -> String {
        format!("Error: {}", e)
    }

    /// Unit tests for the application (optional)
    #[cfg(test)]
    mod tests {
        use super::*;

        #[actix_web::test]
        async fn test_chart_generation() {
            let data = ChartData {
                title: "Test Chart".to_string(),
                labels: vec!["Label 1".to_string(), "Label 2".to_string()],
                values: vec![10.0, 20.0],
            };

            let response = InteractiveChartGenerator::generate_chart(data);
            assert!(response.contains(&data.title));
        }
    }

    // Note: In a real-world scenario, you would need to include chart generation logic
    // and possibly integrate with a charting library or service.