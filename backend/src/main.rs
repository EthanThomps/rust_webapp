use axum::{
    routing::get,
    Router
};









#[tokio::main]
async fn main() {
    // build route
    let app = Router::new().route("/", get(|| async {
        "Route Build"
    }));

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await?
        .unwrap();
}
 