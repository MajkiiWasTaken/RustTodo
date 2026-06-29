use serde::{Deserialize, Serialize};

// Todo is the full object stored by the server.
//
// Serialize - allows Rust to convert Todo into JSON.
//
// Clone - allows us to return a copied Vec<Todo> from GET /todos.
#[derive(Serialize, Deserialize, Clone)]
pub struct Todo { // public for main (using pub)
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub priority: Option<String>, // Optional because the client does not have to send it.
                                  //
                                  // None -> field was not sent.
                                  //
                                  // Some(value) -> update this field.
    pub category: Option<String>,
}

// CreateTodo represents JSON received from the client.
//
// Deserialize - allows Axum/Serde to convert incoming JSON into this Rust struct.
//
// We do not include id or completed here, because those values are created by the server.
#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
    pub priority: Option<String>,
    pub category: Option<String>,
}

// UpdateTodo is used for HTTP PUT requests.
//
// Every field is wrapped in Option<T>.
//
// This allows partial updates.
//
// Example:
//
// {
//     "completed": true
// }
//
// Only "completed" changes.
// All other fields remain unchanged.
#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<String>,
    pub category: Option<String>,
}