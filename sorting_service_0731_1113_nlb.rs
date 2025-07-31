// sorting_service.rs
defining a service using Actix to demonstrate sorting algorithms.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::collections::VecDeque;

// Define a struct to represent the sorting service
struct SortingService;

// Implement functions for sorting algorithms
impl SortingService {
    /// Sorts a vector of integers using bubble sort
    fn bubble_sort(numbers: Vec<i32>) -> Vec<i32> {
        let mut numbers = numbers;
        let len = numbers.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if numbers[j] > numbers[j + 1] {
                    numbers.swap(j, j + 1);
                }
            }
        }
        numbers
    }

    /// Sorts a vector of integers using selection sort
    fn selection_sort(numbers: Vec<i32>) -> Vec<i32> {
        let mut numbers = numbers;
        let len = numbers.len();
        for i in 0..len {
            let mut min_index = i;
            for j in i + 1..len {
                if numbers[j] < numbers[min_index] {
                    min_index = j;
                }
            }
            numbers.swap(i, min_index);
        }
        numbers
    }
}

// Define handlers for HTTP requests
async fn sort_numbers(params: web::Json<Vec<i32>>) -> impl Responder {
    let numbers = params.into_inner();
    let sorted_numbers = SortingService::bubble_sort(numbers); // Change to use selection_sort for selection sort
    HttpResponse::Ok().json(sorted_numbers)
}

// Define the main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sort", web::post().to(sort_numbers))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
