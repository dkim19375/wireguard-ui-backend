use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::data::wireguard_client::WireGuardClientData;
use crate::data::wireguard_server::WireGuardServerData;
use crate::WireGuardAppValues;

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
                    axum::routing::put(put_wireguard_server),
                )
                .route(
                    "/wireguard/clients",
                    axum::routing::get(get_wireguard_clients),
                )
                .route(
                    "/wireguard/clients",
                    axum::routing::put(put_wireguard_clients),
                )
                .route(
                    "/wireguard/clients/:uuid",
                    axum::routing::get(get_wireguard_client),
                )
                .route(
                    "/wireguard/clients/:uuid",
                    axum::routing::put(put_wireguard_client),
                )
                .route("/wireguard/peers", axum::routing::get(get_wireguard_peers))
                .route(
                    "/wireguard/peers/:uuid",
                    axum::routing::get(get_wireguard_peer),
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

async fn put_wireguard_server(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
    Json(body): Json<WireGuardServerData>,
) -> impl IntoResponse {
    let mut app_values = app_values.lock().unwrap();
    app_values.wireguard_data.server = body;
    (StatusCode::OK, "")
}

async fn get_wireguard_clients(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        serde_json::to_string(&app_values.lock().unwrap().wireguard_data.clients).unwrap(),
    )
}

async fn put_wireguard_clients(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
    Json(body): Json<Vec<WireGuardClientData>>,
) -> impl IntoResponse {
    let mut app_values = app_values.lock().unwrap();
    app_values.wireguard_data.clients = body;
    (StatusCode::OK, "")
}

async fn get_wireguard_client(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let app_values = app_values.lock().unwrap();
    match app_values.wireguard_data.get_client_config(&uuid) {
        Some(client) => (StatusCode::OK, serde_json::to_string(&client).unwrap()),
        None => (
            StatusCode::NOT_FOUND,
            format!("Client config for uuid {} not found", uuid),
        ),
    }
}

async fn put_wireguard_client(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
    Path(uuid): Path<Uuid>,
    Json(body): Json<WireGuardClientData>,
) -> impl IntoResponse {
    let mut app_values = app_values.lock().unwrap();
    let client_index = app_values
        .wireguard_data
        .clients
        .iter()
        .position(|client| client.uuid == uuid);
    match client_index {
        Some(index) => app_values.wireguard_data.clients[index] = body,
        None => {
            return (
                StatusCode::NOT_FOUND,
                format!("Client config for uuid {} not found", uuid),
            )
        }
    }
    (StatusCode::OK, String::new())
}

async fn get_wireguard_peers(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
) -> impl IntoResponse {
    let app_values = app_values.lock().unwrap();
    todo!()
}

async fn get_wireguard_peer(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let app_values = app_values.lock().unwrap();
    todo!()
}

async fn wireguard_reload(
    State(app_values): State<Arc<Mutex<WireGuardAppValues>>>,
) -> impl IntoResponse {
    // let mut app_values = app_values.lock().unwrap();
    todo!()
}
