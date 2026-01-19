mod dto;
mod handlers;
mod mappings;
mod security;

use crate::application::ManageServers;
use std::sync::Arc;
use utoipa::OpenApi;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

use self::dto::{CreateDiskRequest, CreateServerRequest, DiskResponse, ServerResponse};
use self::handlers::{handle_attach_disk, handle_create_server, handle_list_servers};
use self::security::{handle_rejection, with_auth};

/// HEXAGONAL ARCHITECTURE: INBOUND ADAPTER (Web)
///
/// --- Good to know ---
/// This module is the entry point for our HTTP transport layer.
///
/// SOLID: This file now acts as a "Composition Root" for the web adapter.
/// It doesn't contain business logic or individual handlers; it only wires
/// them together into a routing table.

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::handle_create_server,
        handlers::handle_list_servers,
        handlers::handle_attach_disk,
    ),
    components(
        schemas(CreateServerRequest, CreateDiskRequest, ServerResponse, DiskResponse)
    ),
    tags(
        (name = "IaaS API", description = "Server management endpoints")
    )
)]
pub struct ApiDoc;

/// Helper to inject the shared Core Service (Port) into our routes.
fn with_port(
    port: Arc<dyn ManageServers>,
) -> impl Filter<Extract = (Arc<dyn ManageServers>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&port))
}

/// Main entry point for the Web API.
/// Orchestrates routes, security, CORS, and OpenAPI spec.
///
/// Comparison:
/// - Go: Like your `RegisterRoutes(router *gin.Engine)` function.
/// - Python: Like the `app = FastAPI()` setup and route registrations.
pub fn routes(
    port: Arc<dyn ManageServers>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // POST /servers
    // We use .and() and other filters to build a declarative "Pipeline".
    let create_server = warp::post()
        .and(warp::path("servers"))
        .and(warp::path::end())
        .and(with_auth()) // Inbound Auth Middleware
        .and(warp::body::content_length_limit(1024 * 16)) // Security: Max Payload
        .and(warp::body::json())
        .and(with_port(Arc::clone(&port))) // Dependency Injection
        .and_then(handle_create_server);

    // GET /servers
    let list_servers = warp::get()
        .and(warp::path("servers"))
        .and(warp::path::end())
        .and(with_auth())
        .and(with_port(Arc::clone(&port)))
        .and_then(handle_list_servers);

    // POST /servers/{id}/disks
    let attach_disk = warp::post()
        .and(warp::path!("servers" / Uuid / "disks"))
        .and(with_auth())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_port(Arc::clone(&port)))
        .and_then(handle_attach_disk);

    // Route for OpenAPI spec
    let openapi_json =
        warp::path!("api-doc" / "openapi.json").map(|| warp::reply::json(&ApiDoc::openapi()));

    // CORS configuration: Inproduction, restrict origins!
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["x-api-key", "content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    let api = create_server
        .or(list_servers)
        .or(attach_disk)
        .or(openapi_json)
        .recover(handle_rejection) // Global Error Handler
        .with(cors);

    // Apply security headers (SRP: logic moved to security.rs)
    security::apply_security_headers(api)
}

#[cfg(test)]
mod tests {
    use self::mappings::map_to_response;
    use super::*;
    use crate::domain::{Disk, ServerStatus};

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
