use uuid::Uuid;

// DTOs for Application Use Cases
// SOLID: Segregated from Web DTOs to avoid coupling.

pub struct CreateServerCommand {
    pub name: String,
    pub cpu: u32,
    pub ram: u32,
    pub storage: u32,
}

pub struct AttachDiskCommand {
    pub server_id: Uuid,
    pub size_gb: u32,
}
