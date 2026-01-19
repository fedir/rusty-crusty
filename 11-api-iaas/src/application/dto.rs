use uuid::Uuid;

/// APPLICATION DTO (Data Transfer Object): CreateServerCommand
///
/// SOLID: This is segregated from the "Web DTO" to avoid coupling our internal
/// business logic to the specific way our web server (Warp) receives JSON.
///
/// Comparison:
/// - Python: Like a dedicated Pydantic class for a Service method.
/// - Go: A custom struct passed into a service function.
pub struct CreateServerCommand {
    pub name: String,
    pub cpu: u32,
    pub ram: u32,
    pub storage: u32,
}

/// APPLICATION DTO: AttachDiskCommand
pub struct AttachDiskCommand {
    pub server_id: Uuid,
    pub size_gb: u32,
}
