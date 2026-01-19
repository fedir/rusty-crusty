use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub storage_gb: u32,
    pub status: ServerStatus,
    pub additional_disks: Vec<Disk>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerStatus {
    Provisioning,
    Running,
    Stopped,
    Terminated,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disk {
    pub id: Uuid,
    pub size_gb: u32,
}

impl Server {
    pub fn new(name: String, cpu: u32, ram: u32, storage: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cpu_cores: cpu,
            ram_gb: ram,
            storage_gb: storage,
            status: ServerStatus::Provisioning,
            additional_disks: Vec::new(),
        }
    }
}
