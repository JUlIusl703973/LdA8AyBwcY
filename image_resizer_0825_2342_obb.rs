use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use image::{open, ImageOutputFormat, imageops::resize};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::prelude::*;
use anyhow::Result;
use log::info;
use simple_logger::SimpleLogger;
use actix_files as fs;

// Struct to hold the configuration for resizing images
struct ResizeConfig {
    width: u32,
    height: u32,
    output_format: ImageOutputFormat,
}

// Function to resize an image to the desired dimensions
fn resize_image(input_path: &Path, output_path: &Path, config: &ResizeConfig) -> Result<()> {
    let img = open(input_path)?;
    let resized_img = resize(&img, config.width, config.height, image::imageops::FilterType::Nearest);

    // Save the resized image to the given path
    resized_img.save_with_format(output_path, config.output_format)?;
    Ok(())
}

// Handler for processing the image resizing request
async fn resize_image_handler(config: web::Data<ResizeConfig>, path: web::Path<String>) -> impl Responder {
    let input_path = Path::new(&path).join("input");
    let output_path = Path::new(&path).join("output");

    if !input_path.exists() {
        return HttpResponse::NotFound().finish();
    }

    match resize_image(&input_path, &output_path, &config) {
        Ok(_) => HttpResponse::Ok().body("Image resized successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error resizing image: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    SimpleLogger::new().init().unwrap();

    // Define the resize configuration
    let resize_config = ResizeConfig {
        width: 800,
        height: 600,
        output_format: ImageOutputFormat::Jpeg,
    };

    // Start the HTTP server with the image resizer handler
    HttpServer::new(move || {
        App::new()
            // Serve static files from the 'static' directory
            .service(fs::Files::new("/static", "static"))
            // Handle image resizing requests
            .service(
                web::resource("/resize/{path}")
                    .guard(actix_web::middleware::Compress::default())
                    .guard(actix_web::middleware::Logger::default())
                    .data(web::Data::new(resize_config))
                    .to(resize_image_handler),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
