// Declare project modules.
//
// Each file (handlers.rs, models.rs, ...) becomes a Rust module
// that can be imported with crate::<module_name>.
mod handlers;
mod models;
mod state;
mod storage;
mod errors;

// Import Axum types used to build the HTTP server.
//
// Router - stores all endpoints.
//
// get(...) - helper used to register GET handlers.
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

use crate::handlers::{
    create_todo, delete_todo, get_todo_by_id, get_todos, update_todo,
};
use crate::state::AppState;
use crate::storage::load_todos;

#[tokio::main]
async fn main() {
    // Create an empty shared todo list.
    let todos: AppState = Arc::new(Mutex::new(load_todos()));

    // Create the router.
    //
    // GET /todos - calls get_todos()
    //
    // POST /todos - calls create_todo()
    //
    // with_state(todos)- attaches our shared Vec<Todo> to the application.
    let app: Router = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo_by_id).delete(delete_todo).put(update_todo))
        .fallback_service(ServeDir::new("static")) // If no API route matches, serve files from the "static" folder.
                                                                                      // Example: GET /index.html
                                                                                      //          GET /style.css
                                                                                      //          GET /script.js
        .with_state(todos);

    // Bind the HTTP server to localhost:3000.
    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // Start the HTTP server.
    // The program waits here for incoming requests.
    axum::serve(listener, app)
        .await
        .unwrap();
}

