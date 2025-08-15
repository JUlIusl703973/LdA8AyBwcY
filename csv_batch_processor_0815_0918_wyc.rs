use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Error as ActixError};
use csv::ReaderBuilder;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

/// Main struct for processing CSV files
struct CsvBatchProcessor;

#[get("/process/{path}")]
async fn process_csv(path: web::Path<String>) -> Result<HttpResponse, ActixError> {
    let path_str = path.into_inner();
    let path = Path::new(&path_str);

    // Check if file exists
    if !path.exists() {
        return Ok(HttpResponse::NotFound().body("File not found"));
    }

    // Read the CSV file
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Failed to open file")),
    };

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut results = vec![];

    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => return Ok(HttpResponse::InternalServerError().body("Failed to parse CSV")),
        };

        // Process each record as needed (example: print record)
        println!("Record: {:?}