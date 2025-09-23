use actix::prelude::*;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use std::env;
use std::sync::Arc;

/// DatabasePoolManager handles the creation and management of the database connection pool.
/// It uses the r2d2 crate to manage multiple connections to a SQLite database.
pub struct DatabasePoolManager;

impl DatabasePoolManager {
    /// Creates a new SQLite connection manager and builds a connection pool.
    pub fn new_pool(database_url: &str) -> Result<r2d2::Pool<SqliteConnectionManager>, r2d2::Error> {
        let manager = SqliteConnectionManager::new(database_url);
        r2d2::Pool::builder()
            .max_size(10) // You can adjust the max size of the pool as needed.
            .build(manager)
    }
}

/// The DatabasePoolActor is responsible for handling requests that require database access.
/// It holds a cloned reference to the database pool, allowing it to run queries.
pub struct DatabasePoolActor {
    pub pool: Arc<r2d2::Pool<SqliteConnectionManager>>,
}

impl DatabasePoolActor {
    /// Creates a new instance of DatabasePoolActor with the specified pool.
    pub fn new(pool: Arc<r2d2::Pool<SqliteConnectionManager>>) -> Self {
        DatabasePoolActor { pool }
    }

    /// Executes a database query using the connection pool.
    pub fn execute_query(&self, query: &str) -> Result<String, r2d2::Error> {
        let conn: PooledConnection<_> = self.pool.get()?;
        let mut stmt = conn.prepare(query)?;
        let result: Vec<String> = stmt.query_map([], |row| {
            row.get(0)
        })?;
        
        Ok(result.into_iter().collect::<Vec<_>>().join(", "))
    }
}

/// Define the Actix handler for the DatabasePoolActor.
impl Actor for DatabasePoolActor {
    type Context = Context<Self>;
}

/// Main function to start the Actix system.
fn main() -> std::io::Result<()> {
    // Load environment variables
    env::set_var("DATABASE_URL", "./your_database.db");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Create the database connection pool
    let pool = DatabasePoolManager::new_pool(&database_url).expect("Failed to create database pool");
    let pool = Arc::new(pool);
    
    // Start the Actix system and spawn the DatabasePoolActor
    System::new().block_on(async move {
        let addr = DatabasePoolActor::new(pool).start();
        
        // Here you can add more logic to interact with the DatabasePoolActor
        // For example, sending messages or running queries.
        
    })?;
    
    Ok(())
}
