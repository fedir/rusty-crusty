use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// --- Inbound DTOs (Request Bodies) ---
///
/// --- Good to know ---
/// DTOs (Data Transfer Objects) in the web layer represent the "External Contract".
/// We use dedicated structs instead of reuse our Domain models to ensure
/// that internal changes don't accidentally break our public API.
///
/// Comparison:
/// - Python: Like a FastAPI/Pydantic request model.
/// - Go: A struct with `json` tags for unmarshaling request bodies.

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
///
/// SOLID: These classes define exactly what we send back to the frontend.
/// They help us avoid "Over-posting" or leaking sensitive internal data.

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
