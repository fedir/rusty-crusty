use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DOMAIN ENTITY: Server
///
/// --- Good to know ---
/// In Hexagonal Architecture, the Domain layer is the "source of truth".
/// It contains the business rules and data structures that define the system.
///
/// Comparison:
/// - Go: Like a 'type Server struct' in your core domain package.
/// - Python: Similar to a Dataclass or a Pydantic model (but without the framework dependencies).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub storage_gb: u32,
    pub status: ServerStatus,
    /// Vector of attached disks. In Rust, Vec<T> is a growable array,
    /// similar to a slice []T in Go or a list [] in Python.
    pub additional_disks: Vec<Disk>,
}

/// DOMAIN ENUM: ServerStatus
///
/// Enum (Enumerations) are extremely powerful in Rust.
/// Unlike Go's 'iota' or Python's 'Enum' class, Rust enums are "Sum Types",
/// meaning they are strictly checked by the compiler.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerStatus {
    Provisioning,
    Running,
    Stopped,
    Terminated,
}

/// DOMAIN ENTITY: Disk
/// Represents a block storage volume that can be attached to a server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disk {
    pub id: Uuid,
    pub size_gb: u32,
}

impl Server {
    /// Factory method to create a new Server with default values.
    /// Notice the 'Provisioning' status is set automatically - this is a "Business Rule".
    pub fn new(name: String, cpu: u32, ram: u32, storage: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cpu_cores: cpu,
            ram_gb: ram,
            storage_gb: storage,
            status: ServerStatus::Provisioning,
            additional_disks: Vec::new(),
        }
    }
}
