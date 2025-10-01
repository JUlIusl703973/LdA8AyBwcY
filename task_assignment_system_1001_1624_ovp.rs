// Task Assignment System using Rust and Actix framework
//
// This system allows for tasks to be assigned and managed.

use actix_web::{
    web,
    get,
    post,
    put,
    delete,
    Error,
    HttpResponse,
# 添加错误处理
    Responder,
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::collections::HashMap;

// Define a structure for a Task
#[derive(Serialize, Deserialize, Debug, Clone)]
# 增强安全性
struct Task {
    id: u32,
# NOTE: 重要实现细节
    title: String,
    description: String,
    assigned_to: String,
}

// Define a structure to handle TaskAssignment
struct TaskAssignment {
    tasks: HashMap<u32, Task>,
}

impl TaskAssignment {
    // Constructor for TaskAssignment
# NOTE: 重要实现细节
    fn new() -> Self {
        TaskAssignment {
            tasks: HashMap::new(),
        }
    }

    // Method to add a new task
    fn add_task(&mut self, task: Task) -> bool {
        if self.tasks.contains_key(&task.id) {
            return false; // Task ID already exists
        }
        self.tasks.insert(task.id, task);
        true
# FIXME: 处理边界情况
    }

    // Method to retrieve a task by ID
# 增强安全性
    fn get_task(&self, task_id: u32) -> Option<&Task> {
        self.tasks.get(&task_id)
    }

    // Method to update a task
    fn update_task(&mut self, task: Task) -> bool {
        if !self.tasks.contains_key(&task.id) {
            return false; // Task ID does not exist
        }
        self.tasks.insert(task.id, task);
        true
    }

    // Method to delete a task
    fn delete_task(&mut self, task_id: u32) -> bool {
        if !self.tasks.contains_key(&task_id) {
            return false; // Task ID does not exist
        }
# 扩展功能模块
        self.tasks.remove(&task_id);
        true
    }
}

// Define the AppState
struct AppState {
    task_assignment: TaskAssignment,
}

#[derive(Serialize, Deserialize)]
struct TaskRequest {
# 增强安全性
    title: String,
    description: String,
    assigned_to: String,
}

// Define the AppError for handling custom errors
#[derive(Debug)]
enum AppError {
    NotFound,
    AlreadyExists,
    InternalServerError,
}

impl Responder for AppError {
    fn respond_to(self, _: &web::HttpRequest) -> HttpResponse {
        match self {
# NOTE: 重要实现细节
            AppError::NotFound => HttpResponse::NotFound().json(json!({"error": "Resource not found"})),
            AppError::AlreadyExists => HttpResponse::BadRequest().json(json!({"error": "Resource already exists"})),
            AppError::InternalServerError => HttpResponse::InternalServerError().json(json!({"error": "Internal server error"})),
        }
    }
}

#[actix_web::main]
# 增强安全性
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        task_assignment: TaskAssignment::new(),
    };

    // Configure the Actix web server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(app_state.clone())
            // Define the routes
            .service(web::resource("/tasks").route(web::post().to(add_task)))
            .service(web::resource("/tasks/{task_id}").route(web::get().to(get_task)))
# 扩展功能模块
            .service(web::resource("/tasks/{task_id}").route(web::put().to(update_task)))
            .service(web::resource("/tasks/{task_id}").route(web::delete().to(delete_task)));
    })
    .bind("127.0.0.1:8080")?
# NOTE: 重要实现细节
    .run()
    .await
# 添加错误处理
}

// Handler for adding a new task
async fn add_task(body: web::Json<TaskRequest>, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let task = Task {
        id: app_state.task_assignment.tasks.len() as u32 + 1,
        title: body.title.clone(),
        description: body.description.clone(),
        assigned_to: body.assigned_to.clone(),
    };
    if app_state.task_assignment.add_task(task) {
        Ok(HttpResponse::Ok().json(json!({"message": "Task added successfully"}