use crate::data::wireguard_server::WireGuardServerData;
use crate::WireGuardAppValues;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

pub async fn start_server(app_values: Arc<Mutex<WireGuardAppValues>>) {
    let address = SocketAddr::from(([0, 0, 0, 0], 6252));
    tokio::spawn(async move {
        let listener = match TcpListener::bind(address).await {
            Ok(listener) => listener,
            #[allow(unused_variables)] // bugged
            Err(error) => {
                panic!("Could not bind to address {address}: {error}");
            }
        };
        let server = axum::serve(
            listener,
            Router::new()
                .route(
                    "/wireguard/server",
                    axum::routing::get(get_wireguard_server),
                )
                .route(
                    "/wireguard/server",
                    axum::routing::patch(patch_wireguard_server),
                )
                .route("/wireguard/reload", axum::routing::post(wireguard_reload))
                .with_state(app_values)
                .into_make_service_with_connect_info::<SocketAddr>(),
        );
        server.await.unwrap();
        panic!("Server stopped unexpectedly");
    });

    println!("Server started on {}", address);
}

async fn get_wireguard_server(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        serde_json::to_string(&app_values.lock().unwrap().wireguard_data).unwrap(),
    )
}

async fn patch_wireguard_server(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
    Json(body): Json<WireGuardServerData>,
) -> impl IntoResponse {
    let mut app_values = app_values.lock().unwrap();
    app_values.wireguard_data.server = body;
    (StatusCode::OK, "")
}

async fn wireguard_reload(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
) -> impl IntoResponse {
    let mut app_values = app_values.lock().unwrap();
    todo!()
}
