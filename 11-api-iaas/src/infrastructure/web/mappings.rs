use super::dto::{DiskResponse, ServerResponse};
use crate::domain::Server;

/// MAPPER PATTERN
///
/// --- Good to know ---
/// This logic exists to transform a "Domain Internal" struct into an "External JSON" DTO.
/// SOLID: By doing this conversion here, our Domain Entities don't need to know
/// anything about how they are presented on the web.
///
/// Comparison:
/// - Python: Like a manual marshmallow schema or a Pydantic `from_orm` logic.
/// - Go: A conversion function like `func ToResponse(s domain.Server) ServerResponse`.
pub fn map_to_response(server: Server) -> ServerResponse {
    ServerResponse {
        id: server.id,
        name: server.name,
        // We convert our internal Enum to a String for the outside world.
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
