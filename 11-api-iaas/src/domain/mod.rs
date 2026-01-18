use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;

/// HEXAGONAL ARCHITECTURE: THE DOMAIN LAYER
/// --- Good to know (Go/Python context) ---
/// In Python, you'd think of this as your "Models" (SQLAlchemy or Pydantic). 
/// In Go, these are your core "Structs" and "Interfaces".
/// 
/// Key Principle: This layer MUST NOT know about the database or the web. 
/// It only contains "Business Logic" and "Entities" (the nouns of your system).
/// ----------------------------------------

/// A Disk entity. 
/// In Rust, we use #[derive(...)] which is like Python's @dataclass 
/// or Go's struct tags. It auto-generates code for things like Debug printing
/// and JSON conversion (Serialize/Deserialize).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Disk {
    pub id: Uuid,
    pub size_gb: u32,
}

/// The core Server entity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub storage_gb: u32,
    pub status: ServerStatus,
    pub additional_disks: Vec<Disk>, // Vec<T> is like a Python List or Go Slice.
}

/// Enums in Rust are "Sum Types". 
/// In Python, you might use a Literal or a String. In Go, you'd use iota.
/// Rust enums ensure you handle all possible states at compile-time.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerStatus {
    Provisioning,
    Running,
    Stopped,
}

/// 'impl' (Implementation) block: 
/// This is where we define methods for the 'Server' struct. 
/// In Python, this would be the methods inside a class. 
/// In Go, these are functions with a 'Server' receiver.
impl Server {
    /// Constructor. By convention, named 'new'.
    /// Similar to __init__ in Python or NewServer in Go.
    pub fn new(name: String, cpu_cores: u32, ram_gb: u32, storage_gb: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cpu_cores,
            ram_gb,
            storage_gb,
            status: ServerStatus::Provisioning,
            additional_disks: Vec::new(),
        }
    }
}

/// HEXAGONAL ARCHITECTURE: OUTBOUND PORT
/// --- Good to know ---
/// This is a "Trait". It's exactly like a Go "Interface".
/// It defines WHAT is needed (e.g., "Save to storage") but not HOW.
/// 
/// The Application layer depends on this interface, allowing us to swap 
/// a File Repository for a SQL Repository without changing business logic.
/// #[async_trait] and 'async fn':
/// Rust's 'async' allows the CPU to do other work while waiting for I/O (like a file or DB).
/// It is similar to Python's 'async/await' or Go's high-concurrency model.
///
/// 'anyhow::Result': Like '(Result, error)' in Go. It simplifies error handling.
#[async_trait]
pub trait ServerRepository: Send + Sync {
    /// async fn is equivalent to Python's 'async def' or Go's goroutine-friendly calls.
    /// anyhow::Result is like a standard Error return in Go or an Exception in Python.
    async fn save(&self, server: &Server) -> anyhow::Result<()>;
    
    /// Retrieves all servers currently stored in the system.
    async fn list_all(&self) -> anyhow::Result<Vec<Server>>;
    
    /// Finds a specific server by its unique UUID. Returns None if not found.
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Server>>; // Option<T> is like T | None in Python.
}
