use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug)]
struct ConverterRequest {
    // The JSON data to be converted
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConverterResponse {
    // The converted JSON data
    converted: String,
}

// This handler will convert the incoming JSON string to a JSON object and return it
#[post("/convert")]
async fn convert_json(data: web::Json<ConverterRequest>) -> impl Responder {
    // Attempt to parse the incoming JSON string into a serde_json::Value
    let parsed_data: Result<Value, _> = serde_json::from_str(&data.data);

    // If the parsing is successful, convert it back to a string and return it
    match parsed_data {
        Ok(parsed_json) => {
            let converted = parsed_json.to_string();
            HttpResponse::Ok().json(ConverterResponse { converted })
        },
        Err(e) => {
            // If there is an error in parsing, return a bad request response with the error message
            HttpResponse::BadRequest().json(json!({"error": e.to_string()}))
        },
    }
}

// This handler will simply return a JSON object with a welcome message
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Welcome to the JSON Converter Service"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/convert", web::post().to(convert_json))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}