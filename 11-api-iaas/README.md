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

## 🔒 Security: OWASP Top 10 API Standards

The project implements several layers of security to demonstrate high-level API protection:

1.  **API-2: Broken Authentication**: Protected endpoints require a valid `x-api-key` header (Standard: `iaas-secret-key-123`).
2.  **API-4: Unrestricted Resource Consumption**: Strictly enforced payload size limits (16KB) on all POST requests to prevent DoS.
3.  **API-8: Security Misconfiguration**:
    *   **Secure Headers**: Implements `X-Content-Type-Options`, `X-Frame-Options`, and `CSP`.
    *   **CORS**: Configured with explicit allowed headers and methods.
    *   **Masked Rejections**: Custom error handlers ensure internal server details aren't leaked in rejections.

---

## � SOLID Principles Implementation

This project is a showcase of the **SOLID** design principles in a modern Rust context:

1.  **S - Single Responsibility**: Every file has one purpose.
    *   `dto.rs` defines the contract.
    *   `security.rs` handles protection.
    *   `handlers.rs` manages the HTTP-to-Core translation.
    *   `mappings.rs` converts data safely.
2.  **O - Open/Closed**: The system is designed for growth. You can add new storage adapters (like SQL) or new transport layers (like gRPC) without touching the heart of the business logic.
3.  **L - Liskov Substitution**: We use Traits (`ManageServers`, `ServerRepository`) to ensure that any implementation can be swapped safely.
4.  **I - Interface Segregation**: Clients only interact with specific traits. The Web layer only sees `ManageServers`, not the internal persistence details.
5.  **D - Dependency Inversion**: The high-level `ServerService` never depends on low-level JSON files. Both depend on the `ServerRepository` abstraction.

---

## �🚀 Getting Started

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
