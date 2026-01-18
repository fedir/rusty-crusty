use crate::domain::{Server, ServerRepository, Disk};
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;

/// HEXAGONAL ARCHITECTURE: INBOUND PORT
/// --- Good to know ---
/// This trait defines the "Use Cases" or "Interactions" our system supports.
/// If someone wants to build a CLI for this API, they would talk to this interface.
/// 
/// In Python, you might call this a "Service Class" interface. In Go, it's a "Standard Service" interface.
#[async_trait]
pub trait ManageServers: Send + Sync {
    async fn create_server(&self, cmd: CreateServerCommand) -> anyhow::Result<Server>;
    async fn list_servers(&self) -> anyhow::Result<Vec<Server>>;
    async fn attach_disk(&self, cmd: AttachDiskCommand) -> anyhow::Result<Server>;
}

/// DTO (Data Transfer Object): Specifically for creating a server.
/// Good to know (Python/Go context): Use these "Command" structs to define exactly what 
/// your use-case needs, rather than passing 10 arguments to a function.
pub struct CreateServerCommand {
    pub name: String,
    pub cpu: u32,
    pub ram: u32,
    pub storage: u32,
}

/// DTO: Specifically for attaching a disk.
pub struct AttachDiskCommand {
    pub server_id: Uuid,
    pub size_gb: u32,
}

/// HEXAGONAL ARCHITECTURE: APPLICATION SERVICE (The "Heart" of the system)
/// 
/// Good to know: In Python/Go, this is where your "Business Rule Logic" lives. 
/// It coordinates work between the Domain (Entities) and Infrastructure (Storage).
pub struct ServerService {
    /// Arc<dyn ServerRepository>: 
    /// - Arc (Atomic Reference Counter) is a thread-safe shared pointer. 
    ///   Think of it as a Go pointer or a Python object reference that can safely 
    ///   live in multiple threads.
    /// - dyn means "Dynamic Dispatch". It tells Rust we're using the Interface (Trait), 
    ///   not a specific concrete class.
    repo: Arc<dyn ServerRepository>,
}

impl ServerService {
    pub fn new(repo: Arc<dyn ServerRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl ManageServers for ServerService {
    /// Use Case: Create Server. 
    /// 1. Create the new entity. 2. Ask the port (repo) to save it. 3. Return result.
    async fn create_server(&self, cmd: CreateServerCommand) -> anyhow::Result<Server> {
        let server = Server::new(cmd.name, cmd.cpu, cmd.ram, cmd.storage);
        // The '?' operator is Rust's way of saying: "If this failed, return the error now."
        // It's much cleaner than Go's 'if err != nil { return err }'.
        self.repo.save(&server).await?;
        println!("Server {} created.", server.id);
        Ok(server)
    }

    async fn list_servers(&self) -> anyhow::Result<Vec<Server>> {
        self.repo.list_all().await
    }

    /// Use Case: Attach Disk.
    async fn attach_disk(&self, cmd: AttachDiskCommand) -> anyhow::Result<Server> {
        // 1. Fetch existing server (or error if not found)
        let mut server = self.repo.find_by_id(cmd.server_id).await?
            .ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        // 2. Perform the domain logic (adding the disk)
        let disk = Disk {
            id: Uuid::new_v4(),
            size_gb: cmd.size_gb,
        };
        server.additional_disks.push(disk);

        // 3. Persist the change
        self.repo.save(&server).await?;
        
        Ok(server)
    }
}
