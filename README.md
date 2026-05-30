# MduX-rust

Medical-device manufacturer framework with Class B/Class C compliance modeling and a Vulkan / Vulkan SC-oriented UI SDK.

## Workspace layout

- `crates/mdux-core`: device metadata, safety classes, deterministic runtime policy
- `crates/mdux-governance`: requirements, hazards, verifications, audit trail, trace matrix export
- `crates/mdux-ui`: Vulkan / Vulkan SC UI policy and deterministic frame model
- `crates/mdux`: thin facade for building complete framework instances
- `examples/hello_world`: smallest out-of-the-box smoke demo
- `examples/class_b_device`: Class B Vulkan example
- `examples/class_c_vulkansc_device`: Class C Vulkan SC example

## Commands

```bash
source $HOME/.cargo/env
cd MduX-rust

# build everything
cargo build

# run all tests
cargo test

# run a single test
cargo test builds_hello_world_demo_through_public_api

# run the shortest demo
cargo run -p hello_world

# run the richer examples
cargo run -p class_b_device
cargo run -p class_c_vulkansc_device
```

The `hello_world` demo uses the current simulated UI runtime, so it proves the framework builds and runs without requiring a Vulkan SDK on the host.
