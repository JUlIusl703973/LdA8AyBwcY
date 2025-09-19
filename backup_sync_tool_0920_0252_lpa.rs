use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

/// The maximum number of files to process in each batch.
const BATCH_SIZE: usize = 10;

/// Handles the backup and sync functionality.
async fn backup_sync() -> impl Responder {
    let source = PathBuf::from("./source");
    let destination = PathBuf::from("./destination");

    // Check if source and destination directories exist.
    if !source.exists() || !destination.exists() {
        return HttpResponse::InternalServerError().json("Directories do not exist.");
    }

    let mut files_to_process = Vec::new();
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            files_to_process.push(path.to_path_buf());
        }
    }

    while !files_to_process.is_empty() {
        let batch = files_to_process.split_off(0..files_to_process.len().saturating_sub(BATCH_SIZE));
        for file_path in batch {
            match process_file(&file_path, &destination) {
                Ok(_) => println!("Processed: {}", file_path.display()),
                Err(e) => println!("Failed to process: {}
Error: {}", file_path.display(), e),
            }
        }
        files_to_process = batch;
    }

    HttpResponse::Ok().json("Backup and sync operation completed.")
}

/// Processes a single file by copying its content to the destination directory.
fn process_file(file_path: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let mut dest_path = destination.join(file_path.strip_prefix(&source)?);
    fs::create_dir_all(dest_path.parent().unwrap())?;
    let mut dest_file = File::create(&dest_path)?;
    dest_file.write_all(&contents)?;
    Ok(())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/backup_sync", web::get().to(backup_sync))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}