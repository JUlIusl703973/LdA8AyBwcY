use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::time::SystemTime;
use actix_web::HttpResponseBuilder;

/// This function is responsible for organizing the folder structure.
/// It will recursively go through each directory and sort files based on their extensions.
async fn organize_folder_structure(path: web::Path<String>) -> impl Responder {
    let path_str = path.into_inner();
    let path = Path::new(&path_str);

    if !path.is_dir() {
        return HttpResponse::BadRequest().body("The provided path is not a directory");
    }

    match sort_directory(path) {
        Ok(_) => HttpResponse::Ok().body("Folder structure has been organized"),
        Err(e) => HttpResponse::InternalServerError().body(format!("An error occurred: {}", e)),
    }
}

/// Sorts directory by moving files into subdirectories based on their extensions.
fn sort_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            sort_directory(path)?;
        } else {
            let extension = match path.extension() {
                Some(ext) => ext.to_str().unwrap_or_default(),
                None => continue,
            };
            let destination = path.with_file_name(format!(".{}", extension));
            fs::rename(path, destination)?;
        }
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(organize_folder_structure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// Define the routes and start the server.
/// All the logic for organizing the folder structure is encapsulated within the `organize_folder_structure` function.
/// This function takes a path as input, checks if it's a directory, and then calls the `sort_directory` function to organize it.
/// Error handling is implemented to ensure that the server responds appropriately to non-directory paths and other I/O errors.
/// The `sort_directory` function recursively sorts files into subdirectories based on their extensions.
/// The main function sets up the Actix web server and starts listening on the specified port.
