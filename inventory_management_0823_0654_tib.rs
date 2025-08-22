use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a struct to represent an inventory item.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// Define a struct to represent the state of the inventory.
struct Inventory {
    items: HashMap<u32, InventoryItem>,
}

impl Inventory {
    // Create a new inventory
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    // Add an item to the inventory
    fn add_item(&mut self, item: InventoryItem) {
        self.items.insert(item.id, item);
    }

    // Get an item from the inventory by id
    fn get_item(&self, id: u32) -> Option<&InventoryItem> {
        self.items.get(&id)
    }

    // Update an item's quantity in the inventory
    fn update_quantity(&mut self, id: u32, quantity: u32) -> Result<&InventoryItem, String> {
        if let Some(item) = self.items.get_mut(&id) {
            item.quantity = quantity;
            Ok(item)
        } else {
            Err("Item not found".to_string())
        }
    }
}

// Define the state data that will be shared across handlers.
#[derive(Clone)]
struct AppState {
    inventory: web::Data<Inventory>,
}

// Define a handler for getting an item by id
async fn get_item_handler(state: web::Data<Inventory>, item_id: web::Path<u32>) -> impl Responder {
    match state.get_item(item_id.into_inner()) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Item not found"),
    }
}

// Define a handler for adding an item
async fn add_item_handler(item: web::Json<InventoryItem>, state: web::Data<Inventory>) -> impl Responder {
    state.add_item(item.into_inner().clone());
    HttpResponse::Created().json(item.into_inner())
}

// Define a handler for updating an item's quantity
async fn update_quantity_handler(item_id: web::Path<u32>, quantity: web::Json<u32>, state: web::Data<Inventory>) -> impl Responder {
    match state.update_quantity(item_id.into_inner(), quantity.into_inner()) {
        Ok(_) => HttpResponse::Ok().json({}),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the inventory
    let mut inventory = Inventory::new();
    // Add some initial items to the inventory for demonstration purposes
    inventory.add_item(InventoryItem { id: 1, name: "Widget".to_string(), quantity: 10 });
    inventory.add_item(InventoryItem { id: 2, name: "Gadget".to_string(), quantity: 20 });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(inventory.clone()))
            .route("/item/{item_id}", web::get().to(get_item_handler))
            .route("/item", web::post().to(add_item_handler))
            .route("/item/{item_id}/quantity", web::put().to(update_quantity_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
