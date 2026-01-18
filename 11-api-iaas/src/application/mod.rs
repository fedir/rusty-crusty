use crate::domain::{Server, ServerRepository};
use std::sync::Arc;

/// HEXAGONAL ARCHITECTURE: APPLICATION LAYER
/// This layer orchestrates the flow of data to and from the domain entities.
/// It uses the "Ports" defined in the domain to talk to the outside world
/// without knowing about specific implementations (like JSON files or SQL).
pub struct ServerService {
    // We use Arc to share the repository across threads safely.
    repo: Arc<dyn ServerRepository>,
}

impl ServerService {
    pub fn new(repo: Arc<dyn ServerRepository>) -> Self {
        Self { repo }
    }

    /// Use Case: Create a new server.
    pub async fn create_server(&self, name: String, cpu: u32, ram: u32, storage: u32) -> anyhow::Result<Server> {
        let server = Server::new(name, cpu, ram, storage);
        self.repo.save(&server).await?;
        println!("Server {} created and saved.", server.id);
        Ok(server)
    }

    /// Use Case: List all servers.
    pub async fn list_servers(&self) -> anyhow::Result<Vec<Server>> {
        self.repo.list_all().await
    }

    /// Use Case: Attach a disk to an existing server.
    pub async fn attach_disk(&self, server_id: uuid::Uuid, size_gb: u32) -> anyhow::Result<Server> {
        let mut server = self.repo.find_by_id(server_id).await?
            .ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        let disk = crate::domain::Disk {
            id: uuid::Uuid::new_v4(),
            size_gb,
        };

        server.additional_disks.push(disk);
        self.repo.save(&server).await?;
        
        println!("Disk attached to server {}.", server.id);
        Ok(server)
    }
}
