use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::io::Write;

// Define a custom error type for document conversion errors
#[derive(Debug)]
enum ConversionError {
    InvalidInput(String),
    IoError(std::io::Error),
}

// Implement Display trait for ConversionError to provide user-friendly error messages
impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ConversionError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

// Implement the error trait for ConversionError
impl std::error::Error for ConversionError {}

// Define a struct to hold the conversion request data
#[derive(Deserialize)]
struct ConversionRequest {
    input: String,
    output_format: String,
}

// Define an asynchronous function to handle document conversion
async fn convert_document(req: web::Json<ConversionRequest>) -> impl Responder {
    // Check for valid input and output format
    if req.input.is_empty() || req.output_format.is_empty() {
        return HttpResponse::BadRequest().json(ConversionError::InvalidInput("Input or output format cannot be empty".to_string()));
    }

    // Perform the conversion logic (this is a placeholder, actual conversion logic should be implemented here)
    let result = match req.output_format.as_str() {
        "pdf" => convert_to_pdf(&req.input),
        "docx" => convert_to_docx(&req.input),
        _ => return HttpResponse::BadRequest().json(ConversionError::InvalidInput("Unsupported output format".to_string())),
    };

    // Handle conversion result
    match result {
        Ok(output) => HttpResponse::Ok().json(output),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

// Placeholder function for converting to PDF (actual implementation needed)
fn convert_to_pdf(input: &str) -> Result<String, ConversionError> {
    // Add actual conversion logic here
    Ok("Converted to PDF".to_string())
}

// Placeholder function for converting to DOCX (actual implementation needed)
fn convert_to_docx(input: &str) -> Result<String, ConversionError> {
    // Add actual conversion logic here
    Ok("Converted to DOCX".to_string())
}

// Define the main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/convert").route(web::post().to(convert_document)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}