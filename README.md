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

# run the shortest demo (opens a Vulkan window)
cargo run -p hello_world

# run it and close automatically after one second
cargo run -p hello_world -- --auto-close-ms=1000

# run the same smoke path without a window
cargo run -p hello_world -- --headless-smoke

# run the richer examples
cargo run -p class_b_device
cargo run -p class_c_vulkansc_device
```

The default `hello_world` example now opens a real Vulkan window. Use `--headless-smoke` when validating the framework in a non-graphical environment.
