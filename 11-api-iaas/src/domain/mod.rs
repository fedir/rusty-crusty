mod entities;
mod repository;

pub use entities::{Disk, Server, ServerStatus};
pub use repository::ServerRepository;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_new() {
        let name = "prod-db-01".to_string();
        let server = Server::new(name.clone(), 8, 32, 500);

        assert_eq!(server.name, name);
        assert_eq!(server.cpu_cores, 8);
        assert_eq!(server.ram_gb, 32);
        assert_eq!(server.storage_gb, 500);
        assert_eq!(server.status, ServerStatus::Provisioning);
        assert!(server.additional_disks.is_empty());
    }
}
