use async_trait::async_trait;
use uuid::Uuid;
use super::entities::Server;

/// HEXAGONAL ARCHITECTURE: OUTBOUND PORT
/// 
/// --- Good to know ---
/// This trait is an "Outbound Port". It defines how the system *wants* to 
/// talk to the outside world (like a Database or a Mock) without knowing 
/// which specific tool is being used.
/// 
/// Comparison:
/// - Go: Exactly like an `interface` in your domain package.
/// - Python: Similar to an Abstract Base Class (ABC).
#[async_trait]
pub trait ServerRepository: Send + Sync {
    /// Save a server's state. In Hexagonal, we don't care if it's JSON or SQL.
    async fn save(&self, server: &Server) -> anyhow::Result<()>;
    
    /// Retrieve all servers currently in storage.
    async fn list_all(&self) -> anyhow::Result<Vec<Server>>;
    
    /// Find a specific server by its unique ID. 
    /// Returns `Option<Server>` which is the Rust way of saying "Maybe it's there, maybe it's not".
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Server>>;
}
