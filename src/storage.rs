use crate::models::Todo;

// Name of the JSON file used as a simple database.
//
// All todos are saved into this file.
const DATA_FILE: &str = "tododata.json";

// Reads the JSON file from disk.
//
// read_to_string() - reads the entire file.
//
// serde_json::from_str() - converts JSON text into Vec<Todo>.
pub fn load_todos() -> Vec<Todo> {
    match std::fs::read_to_string(DATA_FILE){
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()), // If JSON parsing fails,
                                                                                              // return an empty vector instead of crashing.
        Err(_) => Vec::new(), // If the file does not exist,
                              // start with an empty todo list.
    }
}

// Saves the current todo list to disk.
//
// to_string_pretty()
// creates nicely formatted JSON.
pub fn save_todos(todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos).unwrap();
    std::fs::write(DATA_FILE, json).unwrap();
}