use async_trait::async_trait;
use uuid::Uuid;
use super::entities::Server;

#[async_trait]
pub trait ServerRepository: Send + Sync {
    async fn save(&self, server: &Server) -> anyhow::Result<()>;
    async fn list_all(&self) -> anyhow::Result<Vec<Server>>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Server>>;
}
