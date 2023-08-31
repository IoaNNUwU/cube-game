## example rust project
Shows project structure with different modules and crates depending on each other.

Modules:
- client_crates
- server_crates
- common_crates

Each module is a folder that contains specific crates.
Project is organized using `Cargo Workspaces` [see `Cargo.toml`](Cargo.toml)

Run:
- `cargo run --bin server`
- `cargo run --bin client`
