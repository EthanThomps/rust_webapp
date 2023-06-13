use axum::{routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Todos {
    title: &'static str,
    task: &'static str,
    createdAt: u8,
}

#[tokio::main]
async fn main() {
    // ssr solution?
    // https://rust-on-nails.com/docs/full-stack-web/server-side-components/
    
    // build routes
    let app = Router::new()
        .route("/", get(|| async { "index Page" }))
        .route("/create-todos", get(create_todos))
        .route("/delete-todos", get(delete_todos))
        .route("/view-todos", get(view_todos));
    // starts multi-threaded server
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_todos() {
    println!("create-todo page");
    // todo: something from frontend triggers this function
}

async fn delete_todos() {
    println!("delete-todo page");
    // deletes data from db
    // todo: something from frontend triggers this function
}

async fn view_todos(){
    println!("view-todos page");
    // todo: something from frontend triggers this function
}