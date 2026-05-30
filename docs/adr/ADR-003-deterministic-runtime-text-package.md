# ADR-003: Deterministic runtime text package and memory model

## Status

Accepted

## Context

The runtime renderer must guarantee stable glyph submission and avoid heap growth while rendering text in the Vulkan or Vulkan SC UI path.

## Decision

Use an immutable compiled text package containing:

- approved string identifiers
- precomputed glyph runs
- atlas textures and glyph metadata
- numeric token glyph sets and templates
- reproducibility evidence

The runtime shall:

- use fixed-capacity command buffers
- use fixed-point or precomputed glyph positions from the package
- substitute only bounded numeric tokens into approved templates
- reject over-capacity or unknown-token requests explicitly

The runtime shall not perform heap allocation in the draw path and shall not construct new font or shaping objects at runtime.

## Consequences

- Class C rendering stays compatible with offline-compiled Vulkan SC-style constraints.
- Numeric substitution remains possible without turning the runtime into a general-purpose text engine.
- Package schema changes must be controlled because they directly affect the validated runtime contract.
