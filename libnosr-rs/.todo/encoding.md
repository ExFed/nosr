# libnosr-rs Encoding

## Overview

Introduce a nosr document encoder capable of constructing tables, vectors, and scalars programmatically and emitting canonical nosr text, while keeping the feature isolated until stabilized.

## Requirements

- Builders mirror parser navigation APIs for tables, vectors, and scalars.
- Encoder produces valid nosr strings with correct escaping and formatting.
- Span/provenance metadata is retained where feasible, without depending on an input string.
- Public APIs and documentation describe usage, feature gating, and round-trip behavior.
- Encoding surface remains opt-in (feature flag or experimental module) until fully vetted.

## Implementation Steps

- [ ] Story Setup: Draft tracking issue/story, confirm acceptance criteria (parity with parser operations, basic tests, docs).
- [ ] Design Review: Inventory existing Node, Span, and helper traits; identify reusable pieces; decide exposure (module layout, feature flag name).
- [ ] Core Builders: Implement DocumentBuilder, TableBuilder, VectorBuilder, scalar helpers; ensure ownership model (owned strings vs borrowed) supports encoding.
- [ ] Validation Logic: Enforce structural rules (matching braces, key uniqueness, scalar vs container invariants) with meaningful errors.
- [ ] Serialization Engine: Implement emission routines (compact first pass), reuse escaping logic from parser or add shared utility; support pretty-print toggle decision.
- [ ] API Integration: Expose encode_document (and builder methods) through lib.rs; gate behind feature flag; add module and crate docs.
- [ ] Examples and Docs: Add examples/encode.rs demonstrating end-to-end usage; update README and module docs with builder walkthroughs.
- [ ] Round-Trip Support: Provide helper to convert Node into builders for serialize-after-parse workflows; document non-lossy guarantees.
- [ ] Stabilization Prep: Audit types for #[non_exhaustive]/#[repr] where needed; note C FFI considerations for generated output.

## Testing

- [ ] Unit tests for each builder (success paths, duplicate keys, unfinished structures, invalid scalars).
- [ ] Golden encoding tests comparing output strings to expected nosr text (including escaping cases).
- [ ] Round-trip tests: parse sample documents, re-encode, compare structural equality and text normalization.
- [ ] Fuzz/property tests covering builder serialization once core paths stabilize.
- [ ] Doc tests verifying README and module examples compile and run.
