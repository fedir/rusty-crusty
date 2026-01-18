use crate::domain::{Server, ServerRepository};
use async_trait::async_trait;
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;

/// HEXAGONAL ARCHITECTURE: OUTBOUND ADAPTER
/// --- Good to know ---
/// This is the "Driven" side. It's a concrete implementation 
/// of our storage interface (ServerRepository).
/// 
/// In Python, this might be your Django Database Backend or a JSON mock.
/// In Go, this is your repository struct that talks to MySQL or Files.
pub struct JsonServerRepository {
    storage_dir: PathBuf, // PathBuf is like Python's 'pathlib.Path' - it handles OS paths safely.
}

/// 'impl' (Implementation) block for our repository struct.
impl JsonServerRepository {
    /// Creates a new repository instance pointing to the specified directory.
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let storage_dir = PathBuf::from(path);
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }
        Ok(Self { storage_dir })
    }
}

#[async_trait]
/// Implementing the Domain Port (Interface) for our Infrastructure Adapter.
impl ServerRepository for JsonServerRepository {
    /// Serializes and saves the server state to a JSON file.
    async fn save(&self, server: &Server) -> anyhow::Result<()> {
        let file_path = self.storage_dir.join(format!("{}.json", server.id));
        
        // Serialize: Convert Rust Struct -> JSON String.
        // Like json.dumps(server) in Python or json.Marshal(server) in Go.
        let json = serde_json::to_string_pretty(server)?;
        
        fs::write(file_path, json)?;
        Ok(())
    }

    /// Asynchronously loads and parses all JSON server files in the storage directory.
    async fn list_all(&self) -> anyhow::Result<Vec<Server>> {
        let mut servers = Vec::new();
        // Read directory: Like os.listdir() in Python.
        for entry in fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Filter for .json files
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(path)?;
                
                // Deserialize: Convert JSON String -> Rust Struct.
                // Like pydantic.parse_raw() in Python or json.Unmarshal in Go.
                let server: Server = serde_json::from_str(&content)?;
                servers.push(server);
            }
        }
        Ok(servers)
    }

    /// Asynchronously searches for a specific JSON file by server ID and deserializes it.
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Server>> {
        let file_path = self.storage_dir.join(format!("{}.json", id));
        if file_path.exists() {
            let content = fs::read_to_string(file_path)?;
            let server: Server = serde_json::from_str(&content)?;
            Ok(Some(server)) // Found it!
        } else {
            Ok(None) // Not found - perfectly normal in Hexagonal to return an Option.
        }
    }
}
