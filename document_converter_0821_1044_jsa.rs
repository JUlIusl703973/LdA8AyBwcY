use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::io::Read;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use std::error::Error;
use std::fmt::Formatter;

// Define a custom error type for our application
#[derive(Debug)]
struct ConversionError(String);

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ConversionError {}

// Define the request payload structure
#[derive(Deserialize)]
struct ConvertRequest {
    input: String,
    output: String,
}

// Define the handler function for the conversion route
async fn convert_document(req: web::Json<ConvertRequest>) -> impl Responder {
    let input_path = Path::new(&req.input);
    let output_path = Path::new(&req.output);

    // Check if input file exists
    if !input_path.exists() {
        return HttpResponse::BadRequest().json(json!{"error": "Input file does not exist"});
    }

    // Perform the file conversion (dummy implementation)
    let conversion_result = convert_file(&input_path, &output_path).await;

    // Handle conversion result
    match conversion_result {
        Ok(_) => HttpResponse::Ok().json(json!{"message": "File converted successfully"}),
        Err(e) => HttpResponse::InternalServerError().json(json!{"error": e.to_string()}),
    }
}

// Dummy file conversion function (replace with actual implementation)
async fn convert_file(input_path: &Path, output_path: &Path) -> Result<(), ConversionError> {
    // Read the input file
    let mut file = File::open(input_path).map_err(|e| ConversionError(format!("Failed to open input file: {}", e)))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| ConversionError(format!("Failed to read input file: {}", e)))?;

    // Perform the actual file conversion logic here
    // For demonstration, we'll just write the same content to the output file
    let mut output_file = File::create(output_path).map_err(|e| ConversionError(format!("Failed to create output file: {}", e)))?;
    output_file.write_all(contents.as_bytes()).map_err(|e| ConversionError(format!("Failed to write to output file: {}", e)))?;

    // Return Ok on successful conversion
    Ok(())
}

// Define the main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define the server configuration
    HttpServer::new(|| {
        App::new()
            // Register the conversion route with the handler function
            .route("/convert", web::post().to(convert_document))
    })
    // Bind the server to a listening address and start it
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
