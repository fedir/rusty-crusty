use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{Server, ServerRepository, Disk};
use super::ports::ManageServers;
use super::dto::{CreateServerCommand, AttachDiskCommand};

pub struct ServerService {
    repo: Arc<dyn ServerRepository>,
}

impl ServerService {
    pub fn new(repo: Arc<dyn ServerRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl ManageServers for ServerService {
    async fn create_server(&self, cmd: CreateServerCommand) -> anyhow::Result<Server> {
        let server = Server::new(cmd.name, cmd.cpu, cmd.ram, cmd.storage);
        self.repo.save(&server).await?;
        println!("Server {} created.", server.id);
        Ok(server)
    }

    async fn list_servers(&self) -> anyhow::Result<Vec<Server>> {
        self.repo.list_all().await
    }

    async fn attach_disk(&self, cmd: AttachDiskCommand) -> anyhow::Result<Server> {
        let mut server = self.repo.find_by_id(cmd.server_id).await?
            .ok_or_else(|| anyhow::anyhow!("Server not found"))?;

        let disk = Disk {
            id: Uuid::new_v4(),
            size_gb: cmd.size_gb,
        };
        server.additional_disks.push(disk);

        self.repo.save(&server).await?;
        
        Ok(server)
    }
}
