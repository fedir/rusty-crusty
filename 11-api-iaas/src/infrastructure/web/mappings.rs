use super::dto::{DiskResponse, ServerResponse};
use crate::domain::Server;

/// Mapper Pattern:
/// Cleanly separates Domain Entities from Web Presentation DTOs.
pub fn map_to_response(server: Server) -> ServerResponse {
    ServerResponse {
        id: server.id,
        name: server.name,
        status: format!("{:?}", server.status),
        disks: server
            .additional_disks
            .into_iter()
            .map(|d| DiskResponse {
                id: d.id,
                size_gb: d.size_gb,
            })
            .collect(),
    }
}
