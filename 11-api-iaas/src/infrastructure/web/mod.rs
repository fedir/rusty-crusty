/// HEXAGONAL ARCHITECTURE: INBOUND ADAPTER
/// This layer handles incoming HTTP requests and translates them into calls 
/// to the Application Layer (ServerService).
use crate::application::ServerService;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[derive(Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub cpu: u32,
    pub ram: u32,
    pub storage: u32,
}

#[derive(Deserialize)]
pub struct CreateDiskRequest {
    pub size_gb: u32,
}

/// Helper function to provide the shared service to warp filters.
fn with_service(
    service: Arc<ServerService>,
) -> impl Filter<Extract = (Arc<ServerService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&service))
}

pub fn routes(
    service: Arc<ServerService>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let create_server = warp::post()
        .and(warp::path("servers"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_service(Arc::clone(&service)))
        .and_then(handle_create_server);

    let list_servers = warp::get()
        .and(warp::path("servers"))
        .and(warp::path::end())
        .and(with_service(Arc::clone(&service)))
        .and_then(handle_list_servers);

    let attach_disk = warp::post()
        .and(warp::path!("servers" / Uuid / "disks"))
        .and(warp::body::json())
        .and(with_service(Arc::clone(&service)))
        .and_then(handle_attach_disk);

    create_server.or(list_servers).or(attach_disk)
}

/// Handler for creating a new server.
/// Translates the JSON request body into a call to the domain service.
async fn handle_create_server(
    req: CreateServerRequest,
    service: Arc<ServerService>,
) -> Result<impl Reply, Rejection> {
    match service.create_server(req.name, req.cpu, req.ram, req.storage).await {
        Ok(server) => Ok(warp::reply::json(&server)),
        Err(_) => Err(warp::reject::reject()),
    }
}

/// Handler for listing all available servers.
/// Fetches the data from the service and returns it as a JSON array.
async fn handle_list_servers(service: Arc<ServerService>) -> Result<impl Reply, Rejection> {
    match service.list_servers().await {
        Ok(servers) => Ok(warp::reply::json(&servers)),
        Err(_) => Err(warp::reject::reject()),
    }
}

/// Handler for attaching a new disk to an existing server.
/// Uses the server ID from the URL path and the size from the JSON body.
async fn handle_attach_disk(
    server_id: uuid::Uuid,
    req: CreateDiskRequest,
    service: Arc<ServerService>,
) -> Result<impl Reply, Rejection> {
    match service.attach_disk(server_id, req.size_gb).await {
        Ok(server) => Ok(warp::reply::json(&server)),
        Err(_) => Err(warp::reject::reject()),
    }
}
