use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use std::error::Error;

// Define a simple data model for demonstration purposes.
#[derive(Serialize)]
struct MyDataModel {
    id: i32,
    name: String,
    // Additional fields can be added here.
}

// Define the service struct that will handle requests.
struct MyDataService;

// Implement functions for the MyDataService struct.
impl MyDataService {
    #[get("/data-model")]
    async fn get_data_model() -> Result<impl Responder, Box<dyn Error>> {
        let model = MyDataModel {
            id: 1,
            name: "Example Model".to_string(),
        };

        // Simulate potential error with a simple condition.
        let error_condition = false;
        if error_condition {
            return Err("Simulated error".into());
        }

        // Return the serialized model as a JSON response.
        Ok(HttpResponse::Ok().json(model))
    }
}

// Define the main function to start the Actix server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup the Actix web server with the defined service.
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(MyDataService::get_data_model)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
