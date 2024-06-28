use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tokio::net::TcpListener;

pub async fn start_server() {
    let address = SocketAddr::from(([0, 0, 0, 0], 6252));
    tokio::spawn(async move {
        #[allow(unused_variables)] // bugged
        let listener = match TcpListener::bind(address).await {
            Ok(listener) => listener,
            Err(error) => {
                panic!("Could not bind to address {address}: {error}");
            }
        };
        let server = axum::serve(
            listener,
            Router::new()
                .route("/test", axum::routing::get(test_function))
                .into_make_service_with_connect_info::<SocketAddr>(),
        );
        server.await.unwrap();
        panic!("Server stopped unexpectedly");
    });

    println!("Server started on {}", address);
}

async fn test_function() -> impl IntoResponse {
    println!("Got request");
    (
        StatusCode::OK,
        serde_json::to_string(&"Test response").unwrap(),
    )
}
