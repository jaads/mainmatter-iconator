use std::net::SocketAddr;

use iconator::rest_api;

#[tokio::main]
async fn main() {
    let app = rest_api::router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
