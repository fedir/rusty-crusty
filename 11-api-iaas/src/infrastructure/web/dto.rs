use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// --- Inbound DTOs (Request Bodies) ---
// Good to know: These represent the external contract.

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
// These decouple the internal Domain from the external API.

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
