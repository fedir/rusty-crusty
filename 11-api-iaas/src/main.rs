mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;
use crate::application::ServerService;
use crate::infrastructure::persistence::JsonServerRepository;
use crate::infrastructure::web::routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // HEXAGONAL ARCHITECTURE: COMPOSING THE SYSTEM
    // Here we wire the Adapters to the Ports.

    // 1. Initialize Infrastructure (Persistence Adapter)
    let repo = JsonServerRepository::new("./storage")?;
    
    // 2. Initialize Application Service (Dependeny Injection)
    // The service (Inside) only knows about the trait, not the JSON implementation.
    let service = Arc::new(ServerService::new(Arc::new(repo)));
    
    // 3. Define Web Routes
    let api = routes(service);
    
    println!("IaaS Platform API running at http://127.0.0.1:8080");
    println!("- POST /servers : Create a server");
    println!("- GET  /servers : List all servers");
    
    // 4. Start Server
    warp::serve(api)
        .run(([127, 0, 0, 1], 8080))
        .await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::domain::ServerRepository;
    #[tokio::test]
    async fn test_server_creation_persistence() -> anyhow::Result<()> {
        let test_dir = tempdir()?;
        let test_dir_path = test_dir.path().to_str().unwrap();
        let repo = JsonServerRepository::new(test_dir_path)?;
        let service = ServerService::new(Arc::new(repo));

        // Create a server
        let name = "test-vm-01".to_string();
        let server = service.create_server(name.clone(), 4, 16, 250).await?;

        assert_eq!(server.name, name);
        assert_eq!(server.cpu_cores, 4);

        // Verify file exists
        let file_path = test_dir.path().join(format!("{}.json", server.id));
        assert!(file_path.exists());

        // List servers and find our new one
        let all_servers = service.list_servers().await?;
        assert!(all_servers.iter().any(|s| s.id == server.id));

        Ok(())
    }

    #[tokio::test]
    async fn test_disk_attachment() -> anyhow::Result<()> {
        let test_dir = tempdir()?;
        let test_dir_path = test_dir.path().to_str().unwrap();
        let repo = Arc::new(JsonServerRepository::new(test_dir_path)?);
        let service = ServerService::new(repo.clone());

        // Create a server
        let server = service.create_server("vm-disk-test".to_string(), 2, 4, 40).await?;

        // Attach a disk
        let updated_server = service.attach_disk(server.id, 100).await?;

        assert_eq!(updated_server.additional_disks.len(), 1);
        assert_eq!(updated_server.additional_disks[0].size_gb, 100);

        // Reload from persistence and check
        let reloaded = repo.find_by_id(server.id).await?.unwrap();
        assert_eq!(reloaded.additional_disks.len(), 1);
        assert_eq!(reloaded.additional_disks[0].size_gb, 100);

        Ok(())
    }
}
