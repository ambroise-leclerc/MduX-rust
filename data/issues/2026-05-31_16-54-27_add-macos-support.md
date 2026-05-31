## Feature statement 

Add macOS support so Vulkan-based examples (hello_world) and the UI run on macOS using MoltenVK and a documented setup.

## Context 

- The project uses ash + winit + ash_window; examples/hello_world open a Vulkan window (examples/hello_world).
- macOS does not provide a native Vulkan driver: MoltenVK + vulkan-loader (Homebrew) are required to provide a Vulkan ICD.
- Without macOS documentation and CI coverage, contributors cannot run or verify the windowed UI paths on macOS.
- This work focuses on packaging/docs/CI and any small platform conditionals required for macOS.

## Definition of done 

- README documents macOS prerequisites and exact env vars to run examples:
  - brew install vulkan-loader molten-vk vulkan-tools
  - export VK_ICD_FILENAMES=$(brew --prefix)/etc/vulkan/icd.d/MoltenVK_icd.json
  - export DYLD_FALLBACK_LIBRARY_PATH=$(brew --prefix)/lib
- Verified that `cargo run -p hello_world` (windowed or headless smoke) runs on macOS.
- Add a macOS CI job that runs at least the headless smoke; prefer a windowed smoke if the runner supports it.
- Implement any small conditional code changes if required and update tests/docs accordingly.
