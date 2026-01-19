use std::sync::Arc;
use warp::{Rejection, Reply};
use crate::application::{ManageServers, CreateServerCommand, AttachDiskCommand};
use super::dto::{CreateServerRequest, CreateDiskRequest, ServerResponse};
use super::mappings::map_to_response;

#[utoipa::path(
    post,
    path = "/servers",
    request_body = CreateServerRequest,
    responses(
        (status = 200, description = "Server created successfully", body = ServerResponse),
        (status = 400, description = "Invalid request")
    )
)]
pub async fn handle_create_server(
    req: CreateServerRequest,
    port: Arc<dyn ManageServers>,
) -> Result<impl Reply, Rejection> {
    let cmd = CreateServerCommand {
        name: req.name,
        cpu: req.cpu,
        ram: req.ram,
        storage: req.storage,
    };
    
    match port.create_server(cmd).await {
        Ok(server) => Ok(warp::reply::json(&map_to_response(server))),
        Err(_) => Err(warp::reject::reject()),
    }
}

#[utoipa::path(
    get,
    path = "/servers",
    responses(
        (status = 200, description = "List all servers", body = [ServerResponse])
    )
)]
pub async fn handle_list_servers(port: Arc<dyn ManageServers>) -> Result<impl Reply, Rejection> {
    match port.list_servers().await {
        Ok(servers) => {
            let resp: Vec<ServerResponse> = servers.into_iter().map(map_to_response).collect();
            Ok(warp::reply::json(&resp))
        },
        Err(_) => Err(warp::reject::reject()),
    }
}

#[utoipa::path(
    post,
    path = "/servers/{id}/disks",
    request_body = CreateDiskRequest,
    params(
        ("id" = uuid::Uuid, Path, description = "Server UUID")
    ),
    responses(
        (status = 200, description = "Disk attached successfully", body = ServerResponse),
        (status = 404, description = "Server not found")
    )
)]
pub async fn handle_attach_disk(
    server_id: uuid::Uuid,
    req: CreateDiskRequest,
    port: Arc<dyn ManageServers>,
) -> Result<impl Reply, Rejection> {
    let cmd = AttachDiskCommand {
        server_id,
        size_gb: req.size_gb,
    };
    
    match port.attach_disk(cmd).await {
        Ok(server) => Ok(warp::reply::json(&map_to_response(server))),
        Err(_) => Err(warp::reject::reject()),
    }
}
