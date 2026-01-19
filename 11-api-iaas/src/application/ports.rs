use async_trait::async_trait;
use crate::domain::Server;
use super::dto::{CreateServerCommand, AttachDiskCommand};

#[async_trait]
pub trait ManageServers: Send + Sync {
    async fn create_server(&self, cmd: CreateServerCommand) -> anyhow::Result<Server>;
    async fn list_servers(&self) -> anyhow::Result<Vec<Server>>;
    async fn attach_disk(&self, cmd: AttachDiskCommand) -> anyhow::Result<Server>;
}
