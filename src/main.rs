use axum::{extract::{State, Path}, routing::get, Json, Router, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Shared application state.
//
// Arc
//      allows multiple handlers to share ownership of the same data.
//
// Mutex
//      protects the Vec so only one request can access it at a time.
//
// Vec<Todo>
//      stores all todo items in memory.
type AppState = Arc<Mutex<Vec<Todo>>>;

// Todo is the full object stored by the server.
//
// Serialize
//      allows Rust to convert Todo into JSON.
//
// Clone
//      allows us to return a copied Vec<Todo> from GET /todos.
#[derive(Serialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

// CreateTodo represents JSON received from the client.
//
// Deserialize
//      allows Axum/Serde to convert incoming JSON into this Rust struct.
//
// We do not include id or completed here,
// because those values are created by the server.
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
    description: String,
}

#[tokio::main]
async fn main() {
    // Create an empty shared todo list.
    let todos: AppState = Arc::new(Mutex::new(Vec::new()));

    // Create the router.
    //
    // GET /todos
    //      calls get_todos()
    //
    // POST /todos
    //      calls create_todo()
    //
    // with_state(todos)
    //      attaches our shared Vec<Todo> to the application.
    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", get(get_todo_by_id))
        .with_state(todos);

    // Bind the HTTP server to localhost:3000.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // Start the HTTP server.
    // The program waits here for incoming requests.
    axum::serve(listener, app)
        .await
        .unwrap();
}

// Handles GET /todos.
//
// State(todos): State<AppState>
//      extracts the shared application state from Axum.
//
// lock()
//      locks the Mutex so we can safely access the Vec.
//
// clone()
//      creates a copy of the Vec, because the original Vec must stay
//      inside AppState.
async fn get_todos(State(todos): State<AppState>) -> Json<Vec<Todo>> {
    let todos = todos.lock().unwrap();

    Json(todos.clone())
}

// Handles POST /todos.
//
// Json(payload): Json<CreateTodo>
//      extracts JSON from the HTTP request body
//      and converts it into CreateTodo.
//
// This currently creates a Todo and returns it,
// but it does not store it yet.
async fn create_todo(
    State(todos): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = todos.lock().unwrap(); // this says lock the vector but stay mutable
    let id = todos.len() as u32 + 1;

    let todo = Todo {
        id,
        title: payload.title,
        description: payload.description,
        completed: false,
    };

    todos.push(todo.clone()); // push todo copy into vector, we need to push the clone because we 
                              // want to return the original to the client because of the push (passes ownership of the original todo to the vec)

    (StatusCode::CREATED, Json(todo)) // return the original to the client, with the status code CREATED (201)
}

async fn get_todo_by_id(
    State(todos): State<AppState>,
    Path(id): Path<u32>, // takes id from the url
) -> Result<Json<Todo>, StatusCode> {
    
    let todos = todos.lock().unwrap();

    // Search the vector for a Todo with the requested id.
    //
    // iter() - creates an iterator that allows us to read every Todo without taking ownership of the vector.
    //
    // find(...) - stops at the first Todo that matches the condition.
    //
    // |todo| todo.id == id - closure (similar to a lambda in C#) returns true if the current Todo has the requested id.
    //
    // find() - returns Option<&Todo>
    //
    // Some(todo) - a matching Todo was found.
    //
    // None - no Todo with the given id exists.
    match todos.iter().find(|todo| todo.id == id) {
        // Todo found.
        // clone() creates a copy because the original Todo
        // still belongs to the vector.
        Some(todo) => Ok(Json(todo.clone())),
        // Todo not found.
        // Return HTTP 404.
        None => Err(StatusCode::NOT_FOUND),
}
}