# Rust Learning Journey: 11 Sample Projects

A collection of 11 sequentially structured Rust projects designed to take a developer from the absolute basics to advanced concepts like Hexagonal Architecture, Async/Await, and Web Services.

## 🚀 Projects Overview

1.  **[01-cli-basics](./01-cli-basics)**: Introduction to command-line arguments and basic console output.
2.  **[02-guessing-game](./02-guessing-game)**: An interactive game featuring user input, random number generation, and control flow.
3.  **[03-structs-and-methods](./03-structs-and-methods)**: Exploring "Object-Oriented" Rust via structs, implementation blocks, and associated functions.
4.  **[04-enums-and-matching](./04-enums-and-matching)**: Leveraging Rust's powerful `enum` system and `match` expressions.
5.  **[05-file-processing](./05-file-processing)**: Reading from and writing to the local filesystem with robust error handling.
6.  **[06-traits-and-generics](./06-traits-and-generics)**: Understanding polymorphism in Rust through traits and generic type parameters.
7.  **[07-concurrency](./07-concurrency)**: Multi-threading basics including thread spawning and message passing using channels (`mpsc`).
8.  **[08-smart-pointers](./08-smart-pointers)**: Managing memory on the heap and sharing ownership with `Box<T>`, `Rc<T>`, and `RefCell<T>`.
9.  **[09-async-await](./09-async-await)**: Introduction to asynchronous programming using the `Tokio` runtime.
10. **[10-web-server](./10-web-server)**: Building a fast, type-safe HTTP server using the `Warp` framework.
11. **[11-api-iaas](./11-api-iaas)**: A sophisticated "Infrastructure as a Service" (IaaS) API implementing **Hexagonal Architecture** (Ports and Adapters). Features JSON persistence and disk attachment capabilities.

---

## 🛠 Prerequisites

-   **Rust & Cargo**: [Install here](https://rustup.rs/)
-   **System**: Verified on macOS (M4 Pro).

## 🏃 How to Run

Navigate into any project folder and use Cargo:

```bash
cd 11-api-iaas
cargo run
```

To run unit tests:
```bash
cargo test
```

## 🏗 Key Concepts Covered
- **Ownership & Borrowing**: The heart of Rust's safety.
- **Hexagonal Architecture**: Separation of concerns between core logic and infrastructure.
- **RESTful APIs**: Building services with Warp and Serde.
- **Memory Safety**: Moving beyond pointers with smart pointers.
- **Concurrency**: Writing safe, parallel code.

---
*Created as a comprehensive learning resource for modern Rust development.*
