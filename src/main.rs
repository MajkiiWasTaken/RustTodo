use axum::{Router, routing::{self, get}, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Todo{
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

#[tokio::main]
async fn main(){
    // server initialization and startup (tokio = { version = "1.52.3", features = ["full"] } this is needed in cargo toml to work with axum) 
    let app: Router = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
       
    axum::serve(listener,app)
        .await
        .unwrap();

    // from this point we are handling get requests





}

async fn hello() -> &'static str {
    "hello from Rust"
}