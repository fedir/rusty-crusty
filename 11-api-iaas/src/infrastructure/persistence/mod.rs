use crate::domain::{Server, ServerRepository};
use async_trait::async_trait;
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;

/// HEXAGONAL ARCHITECTURE: OUTBOUND ADAPTER
/// This is the Implementation of the Port defined in the domain layer.
/// It handles technical details (FileSystem, JSON serialization).
pub struct JsonServerRepository {
    storage_dir: PathBuf,
}

impl JsonServerRepository {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let storage_dir = PathBuf::from(path);
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }
        Ok(Self { storage_dir })
    }
}

#[async_trait]
impl ServerRepository for JsonServerRepository {
    async fn save(&self, server: &Server) -> anyhow::Result<()> {
        let file_path = self.storage_dir.join(format!("{}.json", server.id));
        let json = serde_json::to_string_pretty(server)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    async fn list_all(&self) -> anyhow::Result<Vec<Server>> {
        let mut servers = Vec::new();
        // Read directory entries
        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(path)?;
                let server: Server = serde_json::from_str(&content)?;
                servers.push(server);
            }
        }
        Ok(servers)
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Server>> {
        let file_path = self.storage_dir.join(format!("{}.json", id));
        if file_path.exists() {
            let content = fs::read_to_string(file_path)?;
            let server: Server = serde_json::from_str(&content)?;
            Ok(Some(server))
        } else {
            Ok(None)
        }
    }
}
