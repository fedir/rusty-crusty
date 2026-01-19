use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{Server, ServerRepository, Disk};
use super::ports::ManageServers;
use super::dto::{CreateServerCommand, AttachDiskCommand};

/// HEXAGONAL ARCHITECTURE: APPLICATION SERVICE
/// 
/// --- Good to know ---
/// This is the "Heart" of the system. It coordinates work between the 
/// Domain logic (Entities) and the Infrastructure (Repository). 
/// It doesn't know *how* things are stored, only *that* they are stored.
/// 
/// Comparison:
/// - Go: Your service implementation struct.
/// - Python: Your business logic controller or service class.
pub struct ServerService {
    /// Dependency Injection: We depend on the Interface (Trait), not a concrete class.
    /// SOLID: This is Dependency Inversion (D) in action.
    repo: Arc<dyn ServerRepository>,
}

impl ServerService {
    /// Factory for creating the service. We "inject" the repository here.
    pub fn new(repo: Arc<dyn ServerRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
/// Implementing our Inbound Port interface.
impl ManageServers for ServerService {
    /// Use Case: Create Server. 
    /// Orchestrates creating the entity and persists it through the repository port.
    async fn create_server(&self, cmd: CreateServerCommand) -> anyhow::Result<Server> {
        let server = Server::new(cmd.name, cmd.cpu, cmd.ram, cmd.storage);
        // We '.await' the port call because persistence might involve I/O.
        self.repo.save(&server).await?;
        println!("Server {} created.", server.id);
        Ok(server)
    }

    /// Use Case: List Servers.
    /// Simply delegates the call to the repository port.
    async fn list_servers(&self) -> anyhow::Result<Vec<Server>> {
        self.repo.list_all().await
    }

    /// Use Case: Attach Disk.
    /// 1. Finds the server. 2. Modifies it. 3. Persists it.
    async fn attach_disk(&self, cmd: AttachDiskCommand) -> anyhow::Result<Server> {
        let mut server = self.repo.find_by_id(cmd.server_id).await?
            .ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        let disk = Disk {
            id: Uuid::new_v4(),
            size_gb: cmd.size_gb,
        };
        server.additional_disks.push(disk);

        // PERSISTENCE: We must call save() again to commit our changes.
        self.repo.save(&server).await?;
        
        Ok(server)
    }
}
