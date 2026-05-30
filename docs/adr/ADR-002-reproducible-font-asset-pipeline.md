# ADR-002: Reproducible font asset pipeline

## Status

Accepted

## Context

Font files, approved strings, Unicode data versions, and atlas-generation rules all influence rendered output. For a safety-oriented SDK, the same approved inputs must lead to the same atlas textures and package bytes.

## Decision

Adopt an explicit host-side asset pipeline with these stages:

1. **Font intake**
   - register source font files
   - record SHA-256 digests, provenance, and approval metadata
2. **Catalog compilation**
   - validate approved/localized strings
   - normalize text and resolve bidi/shaping offline
3. **Atlas generation**
   - rasterize glyphs with a version-pinned toolchain
   - pack glyphs with a deterministic placement algorithm
4. **Package verification**
   - rebuild packages from the same manifest
   - compare atlas hashes and final package hashes byte-for-byte

Tooling inputs must be fully pinned: Rust toolchain, `Cargo.lock`, Unicode data version, source font digests, and package schema version.

## Consequences

- Rebuilds can produce evidence that text assets are reproducible.
- Unapproved font changes are caught as manifest or hash mismatches.
- Atlas layout must avoid nondeterministic heuristics such as hash-order-driven packing.
