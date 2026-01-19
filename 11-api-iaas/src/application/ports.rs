use async_trait::async_trait;
use crate::domain::Server;
use super::dto::{CreateServerCommand, AttachDiskCommand};

/// HEXAGONAL ARCHITECTURE: INBOUND PORT
/// 
/// --- Good to know ---
/// This trait is the "Inbound Port" (or Driving Port). It defines the use-cases
/// that our application supports (creating servers, etc.). 
/// 
/// If you want to add a CLI later, the CLI would talk to this interface,
/// exactly like the Web API does now.
/// 
/// Comparison:
/// - Go: Like a "Service" interface definition.
/// - Python: An Abstract Base Class for your application services.
#[async_trait]
pub trait ManageServers: Send + Sync {
    async fn create_server(&self, cmd: CreateServerCommand) -> anyhow::Result<Server>;
    async fn list_servers(&self) -> anyhow::Result<Vec<Server>>;
    async fn attach_disk(&self, cmd: AttachDiskCommand) -> anyhow::Result<Server>;
}
