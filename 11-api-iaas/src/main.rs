mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;
use crate::application::{ServerService, ManageServers};
use crate::infrastructure::persistence::JsonServerRepository;
use crate::infrastructure::web::routes;

/// THE ENTRY POINT
/// --- Good to know ---
/// In Go, this is your 'func main()'. In Python, your 'if __name__ == "__main__":'.
/// 
/// This is the "Composition Root". Its only job is to:
/// 1. Create the database connection (Repository).
/// 2. Create the application core (Service).
/// 3. Wire them together (Dependency Injection).
/// 4. Start the HTTP server.
/// #[tokio::main]: Rust doesn't have a built-in async runtime like Go.
/// We use 'Tokio' as the engine to run our 'async' tasks. 
/// It's the industry standard for high-performance networking in Rust.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    // 1. Initialize Infrastructure (The OUTSIDE world)
    let repo = JsonServerRepository::new("./storage")?;
    
    // 2. Initialize Application Core (The INSIDE world)
    // Dependency Injection: We create the Service and "inject" the repository into it.
    // In Python, you'd just pass the repo to the constructor. 
    // In Go, you'd pass a struct that satisfies the interface.
    // In Rust, we wrap it in Arc (Atomic Reference Counter) so it can be shared safely with the web server.
    let service: Arc<dyn ManageServers> = Arc::new(ServerService::new(Arc::new(repo)));
    
    // 3. Setup the Driving Adapter (The WEB server)
    let api = routes(service);
    
    println!("IaaS Platform API running at http://127.0.0.1:8080");
    println!("- POST /servers : Create a server");
    println!("- GET  /servers : List all servers");
    
    // 4. Start Server: This is a blocking call (Infinite loop).
    warp::serve(api)
        .run(([127, 0, 0, 1], 8080))
        .await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::application::{CreateServerCommand, AttachDiskCommand};
    
    /// Integration Test: Verifies that the whole chain (Core -> Repo -> Filesystem) works.
    #[tokio::test]
    async fn test_server_creation_persistence() -> anyhow::Result<()> {
        let test_dir = tempdir()?;
        let test_dir_path = test_dir.path().to_str().unwrap();
        
        // Setup infrastructure for test
        let repo = JsonServerRepository::new(test_dir_path)?;
        let service: Arc<dyn ManageServers> = Arc::new(ServerService::new(Arc::new(repo)));

        // 1. Create a server
        let cmd = CreateServerCommand {
            name: "test-vm-01".to_string(),
            cpu: 4,
            ram: 16,
            storage: 250,
        };
        let server = service.create_server(cmd).await?;

        // 2. Verify: Check if the file was actually written to the temp directory (Outbound check)
        let file_path = test_dir.path().join(format!("{}.json", server.id));
        assert!(file_path.exists());

        // 3. Verify: Check if it shows up in the list (Inbound check)
        let all_servers = service.list_servers().await?;
        assert!(all_servers.iter().any(|s| s.id == server.id));

        Ok(())
    }

    /// Integration Test: Verifies that attaching a disk persists the state correctly.
    #[tokio::test]
    async fn test_disk_attachment() -> anyhow::Result<()> {
        let test_dir = tempdir()?;
        let test_dir_path = test_dir.path().to_str().unwrap();
        let repo_impl = Arc::new(JsonServerRepository::new(test_dir_path)?);
        let service: Arc<dyn ManageServers> = Arc::new(ServerService::new(repo_impl.clone()));

        // Create
        let create_cmd = CreateServerCommand {
            name: "vm-disk-test".to_string(),
            cpu: 2,
            ram: 4,
            storage: 40,
        };
        let server = service.create_server(create_cmd).await?;

        // Attach
        let attach_cmd = AttachDiskCommand {
            server_id: server.id,
            size_gb: 100,
        };
        let updated_server = service.attach_disk(attach_cmd).await?;

        // Assert
        assert_eq!(updated_server.additional_disks.len(), 1);
        assert_eq!(updated_server.additional_disks[0].size_gb, 100);

        Ok(())
    }

    /// Unit/Integration Test: Verifies that the OpenAPI spec is generated and exposed correctly.
    #[tokio::test]
    async fn test_openapi_spec_exposure() -> anyhow::Result<()> {
        let test_dir = tempdir()?;
        let test_dir_path = test_dir.path().to_str().unwrap();
        let repo = Arc::new(JsonServerRepository::new(test_dir_path)?);
        let service: Arc<dyn ManageServers> = Arc::new(ServerService::new(repo));
        
        let api = routes(service);

        // Request the OpenAPI JSON
        let resp = warp::test::request()
            .method("GET")
            .header("x-api-key", "iaas-secret-key-123")
            .path("/api-doc/openapi.json")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), 200);
        
        // Verify it contains our main endpoints
        let body_str = std::str::from_utf8(resp.body()).unwrap();
        assert!(body_str.contains("/servers"));
        assert!(body_str.contains("IaaS API"));
        assert!(body_str.contains("CreateServerRequest"));

        Ok(())
    }

    /// Security Test: Verifies that missing API Key results in 401 Unauthorized.
    #[tokio::test]
    async fn test_security_unauthorized() -> anyhow::Result<()> {
        let test_dir = tempdir()?;
        let test_dir_path = test_dir.path().to_str().unwrap();
        let repo = Arc::new(JsonServerRepository::new(test_dir_path)?);
        let service: Arc<dyn ManageServers> = Arc::new(ServerService::new(repo));
        let api = routes(service);

        // Request WITHOUT the x-api-key header
        let resp = warp::test::request()
            .method("GET")
            .path("/servers")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), 401);
        Ok(())
    }
}
