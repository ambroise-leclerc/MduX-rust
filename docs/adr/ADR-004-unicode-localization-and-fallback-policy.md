# ADR-004: Unicode, localization, and fallback policy

## Status

Accepted

## Context

The product needs full Unicode, shaping, and bidi coverage for approved/localized strings, but the safety-critical runtime must remain deterministic and bounded.

## Decision

- Full Unicode support is provided by the **offline** pipeline.
- Every shipped locale must declare its approved strings and required glyph coverage.
- Fallback fonts are permitted only when they are:
  - listed in the manifest
  - hash-pinned
  - coverage-verified
  - selected by deterministic priority order
- Uncovered code points are compilation errors, not runtime warnings.
- Runtime-provided arbitrary Unicode strings are out of scope for the safety-critical runtime.

Dynamic runtime text is limited to bounded numeric tokens inserted into approved templates.

## Consequences

- Localization review moves into the authoring process, where it can be audited and reproduced.
- There is no silent degradation path for missing glyphs or fallback selection.
- The runtime claim stays honest: it renders approved packages, not arbitrary Unicode input.
