/// HEXAGONAL ARCHITECTURE: INBOUND ADAPTER
/// --- Good to know ---
/// This is the "Driving" side of the hexagon. It takes inputs from the outside 
/// (HTTP requests) and translates them into a format the application core understands (Commands).
/// 
/// Comparison:
/// - In Go (Gin/Echo): This is where your 'Handlers' live.
/// - In Python (FastAPI/Flask): These are your 'Routes' or 'Views'.
use crate::application::{ManageServers, CreateServerCommand, AttachDiskCommand};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use utoipa::{OpenApi, ToSchema};
use warp::{Filter, Rejection, Reply};

// --- Inbound DTOs (Request Bodies) ---
// Just like Pydantic models in Python or JSON struct tags in Go.

#[derive(Deserialize, ToSchema)]
pub struct CreateServerRequest {
    pub name: String,
    pub cpu: u32,
    pub ram: u32,
    pub storage: u32,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateDiskRequest {
    pub size_gb: u32,
}

// --- Outbound DTOs (Response Bodies) ---
// Good to know: Why not return the Domain Entity directly? 
// Answer: Decoupling. We don't want to break our API just because we changed 
// how our internal Server struct looks. This is a common pattern in big Go/Python projects too.

#[derive(Serialize, ToSchema)]
pub struct ServerResponse {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub disks: Vec<DiskResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct DiskResponse {
    pub id: Uuid,
    pub size_gb: u32,
}

/// Helpers to inject the shared Core Service (Port) into our routes.
fn with_port(
    port: Arc<dyn ManageServers>,
) -> impl Filter<Extract = (Arc<dyn ManageServers>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&port))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handle_create_server,
        handle_list_servers,
        handle_attach_disk,
    ),
    components(
        schemas(CreateServerRequest, CreateDiskRequest, ServerResponse, DiskResponse)
    ),
    tags(
        (name = "IaaS API", description = "Server management endpoints")
    )
)]
pub struct ApiDoc;

/// Routing setup in Warp is "Functional" and uses "Filters".
/// Good to know (Go/Python context): Think of this as a chain of middleware. 
/// Each '.and()' is like a piece of middleware that must pass.
pub fn routes(
    port: Arc<dyn ManageServers>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    
    // POST /servers
    let create_server = warp::post()
        .and(warp::path("servers"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_port(Arc::clone(&port)))
        .and_then(handle_create_server);

    // GET /servers
    let list_servers = warp::get()
        .and(warp::path("servers"))
        .and(warp::path::end())
        .and(with_port(Arc::clone(&port)))
        .and_then(handle_list_servers);

    // POST /servers/{id}/disks
    let attach_disk = warp::post()
        .and(warp::path!("servers" / Uuid / "disks"))
        .and(warp::body::json())
        .and(with_port(Arc::clone(&port)))
        .and_then(handle_attach_disk);

    // --- OpenAPI ---
    
    // Route to expose the OpenAPI JSON
    let openapi_json = warp::path!("api-doc" / "openapi.json")
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    create_server
        .or(list_servers)
        .or(attach_disk)
        .or(openapi_json)
}

#[utoipa::path(
    post,
    path = "/servers",
    request_body = CreateServerRequest,
    responses(
        (status = 200, description = "Server created successfully", body = ServerResponse),
        (status = 400, description = "Invalid request")
    )
)]
/// Handler: Like a Controller method in Python or a Handler function in Go.
/// 'async fn': Handlers are asynchronous because they wait for the Core logic 
/// (which might wait for the Database) without blocking the thread.
async fn handle_create_server(
    req: CreateServerRequest,
    port: Arc<dyn ManageServers>,
) -> Result<impl Reply, Rejection> {
    // Translate the Web Request into an Application Command.
    let cmd = CreateServerCommand {
        name: req.name,
        cpu: req.cpu,
        ram: req.ram,
        storage: req.storage,
    };
    
    // '.await' tells Rust to pause here until the operation completes, 
    // letting the CPU do other work in the meantime. 
    // Similar to 'await' in JavaScript/Python.
    match port.create_server(cmd).await {
        // Translate the Domain Result back into a Web Response (JSON).
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
/// Handler for listing all servers. Translates the results into Web Response DTOs.
async fn handle_list_servers(port: Arc<dyn ManageServers>) -> Result<impl Reply, Rejection> {
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
        ("id" = Uuid, Path, description = "Server UUID")
    ),
    responses(
        (status = 200, description = "Disk attached successfully", body = ServerResponse),
        (status = 404, description = "Server not found")
    )
)]
/// Handler for disk attachment requests. 
/// Extracts the server ID from the path and the disk info from the request body.
async fn handle_attach_disk(
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

/// Mapper Pattern: 
/// Good to know: In Python, you might use a Marshmallow schema or Pydantic. 
/// In Go, you'd have a manual conversion function. 
/// This keeps your Web and Domain models cleanly separated.
fn map_to_response(server: crate::domain::Server) -> ServerResponse {
    ServerResponse {
        id: server.id,
        name: server.name,
        status: format!("{:?}", server.status),
        disks: server.additional_disks.into_iter().map(|d| DiskResponse {
            id: d.id,
            size_gb: d.size_gb,
        }).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ServerStatus, Disk};

    #[test]
    fn test_map_to_response() {
        let server = crate::domain::Server {
            id: Uuid::new_v4(),
            name: "test-vm".to_string(),
            cpu_cores: 2,
            ram_gb: 4,
            storage_gb: 40,
            status: ServerStatus::Running,
            additional_disks: vec![Disk {
                id: Uuid::new_v4(),
                size_gb: 100,
            }],
        };

        let response = map_to_response(server.clone());

        assert_eq!(response.id, server.id);
        assert_eq!(response.name, server.name);
        assert_eq!(response.status, "Running");
        assert_eq!(response.disks.len(), 1);
        assert_eq!(response.disks[0].size_gb, 100);
    }
}
