# 11-api-iaas: Hexagonal Architecture in Rust

This project is a sophisticated example of an **Infrastructure-as-a-Service (IaaS)** API built with Rust. It is specifically designed as a learning resource for developers transitioning from **Go** or **Python**, emphasizing clean code, architectural patterns, and modern API documentation.

## 🏗️ Architectural Pattern: Hexagonal Architecture

The project strictly follows the **Hexagonal Architecture** (also known as Ports and Adapters) pattern to ensure high testability and decoupling from external concerns.

### 1. Domain Layer (`src/domain/`)
The "Heart" of the system.
- **Entities**: `Server`, `Disk`, `ServerStatus`.
- **Outbound Port**: `ServerRepository` trait (Interface).
- **Rules**: Pure business logic. Zero dependencies on web frameworks or databases.

### 2. Application Layer (`src/application/`)
The Orchestrator.
- **Service**: `ServerService` implements the business use cases.
- **Inbound Port**: `ManageServers` trait.
- **DTOs**: `CreateServerCommand`, `AttachDiskCommand` (Input objects).

### 3. Infrastructure Layer (`src/infrastructure/`)
The Outside World.
- **Persistence (Outbound Adapter)**: `JsonServerRepository` implements disk-based storage using JSON files.
- **Web (Inbound Adapter)**: `warp` based HTTP Handlers that translate requests into Application Commands.

---

## 💡 "Good to Know" (Context for Go/Python Developers)

Throughout the codebase, you will find comments labeled `/// Good to know` which provide direct comparisons to Go and Python concepts:

| Concept | Rust Equivalent | Go Comparison | Python Comparison |
| :--- | :--- | :--- | :--- |
| **Interface** | `Trait` | `interface` | Abstract Base Class |
| **Logic Container**| `impl` Block | Functions with Receivers | Class Methods |
| **Error Handling** | `?` Operator | `if err != nil` | `try/except` |
| **Shared State** | `Arc<T>` | Thread-safe pointers | Object references |
| **Serialization** | `serde` | `json.Marshal` | `pydantic` / `marshmallow` |
| **Runtime** | `Tokio` | Built-in Goroutine engine | `asyncio` loop |

---

## 📖 OpenAPI & Documentation

The project uses `utoipa` to generate an **OpenAPI 3.0 specification** at compile-time directly from documentation comments and Rust types.

- **OpenAPI JSON**: Available at `GET /api-doc/openapi.json`
- **Frontend Sync**: This spec can be used to auto-generate frontend clients or types (TypeScript, etc.).

---

## 🚀 Getting Started

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) installed.

### Run the API
```bash
cd 11-api-iaas
cargo run
```
The server will start at `http://127.0.0.1:8080`.

### API Endpoints
- `POST /servers`: Create a new virtual server.
- `GET /servers`: List all provisioned servers.
- `POST /servers/{id}/disks`: Attach a new disk volume to a server.
- `GET /api-doc/openapi.json`: Download the OpenAPI specification.

---

## 🧪 Testing

The project includes unit tests for domain logic and full integration tests for the API surface.

```bash
cargo test
```

- **Domain Tests**: Verify entity construction and status defaults.
- **Integration Tests**: Verify the full path from HTTP Request -> Application Logic -> JSON File Storage.
- **Spec Tests**: Ensure the OpenAPI specification is correctly generated and served.

---

## 🛠️ Technology Stack
- **Web**: `warp` (Filters-based functional routing)
- **Async**: `tokio` (Industry-standard runtime)
- **Serialization**: `serde` & `serde_json`
- **Error Handling**: `anyhow`
- **Documentation**: `utoipa` (OpenAPI)
