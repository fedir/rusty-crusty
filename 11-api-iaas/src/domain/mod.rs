use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;

/// HEXAGONAL ARCHITECTURE: 
/// This is the DOMAIN layer (the Inner Core).
/// It contains the business logic, entities, and rules that are independent of any external technology.
/// It defines "Ports" (Traits) that the external world must satisfy to interact with the core.

/// A Disk entity represents an additional storage volume.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Disk {
    pub id: Uuid,
    pub size_gb: u32,
}

/// The core Server entity representing a virtual or physical machine in our IaaS.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub storage_gb: u32,
    pub status: ServerStatus,
    pub additional_disks: Vec<Disk>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerStatus {
    Provisioning,
    Running,
    Stopped,
}

impl Server {
    /// Creates a new Server instance with default Provisioning status and no additional disks.
    pub fn new(name: String, cpu_cores: u32, ram_gb: u32, storage_gb: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cpu_cores,
            ram_gb,
            storage_gb,
            status: ServerStatus::Provisioning,
            additional_disks: Vec::new(),
        }
    }
}

/// HEXAGONAL ARCHITECTURE: OUTBOUND PORT
/// This trait defines what the domain needs from the outside world (storage).
/// The infrastructure layer will implement this trait (Adapter).
#[async_trait]
pub trait ServerRepository: Send + Sync {
    /// Persists a server entity to the storage medium.
    async fn save(&self, server: &Server) -> anyhow::Result<()>;
    
    /// Retrieves all persisted servers.
    async fn list_all(&self) -> anyhow::Result<Vec<Server>>;
    
    /// Attempts to find a single server by its unique identifier.
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Server>>;
}
