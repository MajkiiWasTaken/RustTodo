use std::sync::{Arc, Mutex};

use crate::models::Todo;

// Shared application state.
//
// Arc - allows multiple handlers to share ownership of the same data.
//
// Mutex - protects the Vec so only one request can access it at a time.
//
// Vec<Todo> - stores all todo items in memory.
pub type AppState = Arc<Mutex<Vec<Todo>>>;
