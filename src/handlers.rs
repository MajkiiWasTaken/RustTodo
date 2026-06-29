use axum::{extract::{State, Path}, Json, http::StatusCode};

use crate::models::{CreateTodo, Todo, UpdateTodo};
use crate::state::AppState;
use crate::storage::save_todos;
use crate::errors::ApiError;

// Handles GET /todos.
//
// State(todos): State<AppState> - extracts the shared application state from Axum.
//
// lock() - locks the Mutex so we can safely access the Vec.
//
// clone() - creates a copy of the Vec, because the original Vec must stay inside AppState.
pub async fn get_todos(State(todos): State<AppState>) -> Json<Vec<Todo>> {
    let todos = todos.lock().unwrap();

    Json(todos.clone())  // todo is only a mutable reference (&mut Todo).
                         //
                         // We cannot move the original Todo out of the vector.
                         //
                         // clone() creates an owned copy that can be returned.
}

// Handles POST /todos.
//
// Json(payload): Json<CreateTodo> - extracts JSON from the HTTP request body and converts it into CreateTodo.
//
// This currently creates a Todo and returns it, but it does not store it yet.
pub async fn create_todo(
    State(todos): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    let mut todos = todos.lock().unwrap(); // this says lock the vector but stay mutable, 
                                                                      //lock() blocks until the mutex becomes available, 
                                                                      //unwrap() extracts the MutexGuard.
    let id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1; // find the max id in the vector and add 1 to it, if the vector is empty then return 0

    if payload.title.trim().is_empty() {
        return Err(ApiError::BadRequest("Title cannot be empty".to_string()));
    }

    let todo = Todo {
        id,
        title: payload.title,
        description: payload.description,
        completed: false,
        priority: payload.priority,
        category: payload.category,
    };

    todos.push(todo.clone());       // push() takes ownership of the Todo.
                                    //
                                    // If we wrote:
                                    //
                                    // todos.push(todo);
                                    //
                                    // then todo would be moved into the vector and we could no longer return it.
                                    //
                                    // clone() creates a second Todo.
                                    //
                                    // Original -> returned to the client.
                                    //
                                    // Clone -> stored in the vector.
                                    

    save_todos(&todos);       // Persist changes immediately.
                              //
                              // Without this call, changes would disappear after restarting the server.

    Ok((StatusCode::CREATED, Json(todo))) // return the original to the client, with the status code CREATED (201)
}

pub async fn get_todo_by_id(
    State(todos): State<AppState>,
    Path(id): Path<u32>, // takes id from the url
) -> Result<Json<Todo>, ApiError> {
    
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
        None => Err(ApiError::NotFound(format!("Todo with id {} not found", id))),
}
}

pub async fn delete_todo(
    State(todos): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, ApiError> {
    let mut todos = todos.lock().unwrap();

    let old_len = todos.len();

    todos.retain(|todo| todo.id != id);

    if todos.len() < old_len {
        save_todos(&todos);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound(format!("Todo with id {} not found", id)))
    }
}

pub async fn update_todo(
    State(todos): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    let mut todos = todos.lock().unwrap();

    match todos.iter_mut().find(|todo| todo.id == id) {   // iter_mut() returns mutable references.
                                                                      //
                                                                      // We need mutable references because we are changing the Todo.
        Some(todo) => {
            if let Some(title) = payload.title { // check if title is Some, if it is then update the title 
                todo.title = title;
            }

            if let Some(description) = payload.description {
                todo.description = description;
            }

            if let Some(completed) = payload.completed {
                todo.completed = completed;
            }

            if let Some(priority) = payload.priority {
                todo.priority = Some(priority);
            }

            if let Some(category) = payload.category {
                todo.category = Some(category);
            }

            let updated_todo = todo.clone();   // Create a copy before returning it.
                                                     //
                                                     // The original Todo must remain inside the vector.

            save_todos(&todos);

            Ok(Json(updated_todo))
        }
        None => Err(ApiError::NotFound(format!("Todo with id {} not found", id))),
    }
}

