# libnosr-rs Implementation Plan

## Overview

Build `libnosr-rs`, a Rust crate that parses nosr documents into a navigable
tree and offers the same high-level API (`document`, `tab`, `vec`, `text`,
`uint64`, `double`, etc.) promised by the specification, while staying
idiomatic, ergonomic, efficient, and easy to understand as a reference
implementation.

## Requirements

- Keep your implementations in the simplest form that is still correct, keep
  them well-commented throughout, and document design choices so newcomers can
  learn from the code.
- Support UTF-8 nosr input, including tables `{}`, vectors `[]`, scalars, quoted
  text with escapes, whitespace, and both comment styles.
- Represent parsed data as immutable nodes referencing source spans, enabling
  deferred parsing.
- Provide ergonomic result/option types with precise error context (location,
  expectation).
- Match the API surface described: lazy document loading, table/vector
  navigation, scalar conversion helpers.
- Offer optional feature flags for extra conversions (e.g., rationals) without
  bloating the core.
- Supply documentation and examples reflecting the README rationale.
- Builders must mirror parser navigation for tables, vectors, and scalars.
- Encoder output must escape and format strings according to the nosr rules.
- Span metadata should be retained where possible without requiring inputs.
- Public APIs and docs must cover usage, feature gating, and round-trip flows.

## Features

- [ ] `TryFrom` trait implementations for ergonomic `Node` conversions.
- [ ] A base64 operation for binary data extraction.
- [ ] Exploration into optional feature flags for extended numeric or rational
  support.
- [ ] Expand numeric helpers (i8, i16, i32, i64, u8, u16, u32, bool) and gate
  extras behind feature flags.
- [ ] Exploration into a way to eagerly parse a nosr document into a
  fully-realized tree structure, rather than the current lazy node model.
- [ ] Fuzz and property tests for parser and forthcoming encoder paths.
- [ ] Profile and optimize hot paths before wider adoption.
- [ ] Additional examples with more complex usage scenarios.
- [ ] Plan for C FFI bindings for interoperability with existing consumers.
- [ ] Optional Serde integration guarded by a feature flag.
- [ ] Builder support. See [Builder Feature](#builder-feature) below.

## Builder Feature

A nosr document encoder that mirrors the parser ergonomics and emits canonical
nosr text while remaining opt-in until stabilized.

### Implementation Steps

- [ ] Story Setup: Draft a tracking story and confirm acceptance criteria that
  match parser parity, basic tests, and documentation.
- [ ] Design Review: Survey existing `Node`, `Span`, and helper traits; define
  reusable pieces and the feature flag surface.
- [ ] Core Builders: Implement `DocumentBuilder`, `TableBuilder`,
  `VectorBuilder`, and scalar helpers with an ownership model that suits
  encoding.
- [ ] Validation Logic: Enforce structural rules such as matching braces, key
  uniqueness, and scalar versus container constraints with clear errors.
- [ ] Serialization Engine: Implement emission routines, reuse parser escaping
  utilities, and select an initial formatting style.
- [ ] API Integration: Expose `encode_document` and builders from `lib.rs`,
  guarded by the feature flag, and document the module.
- [ ] Examples and Docs: Add `examples/encode.rs` and update references to show
  builder workflows end to end.
- [ ] Round-Trip Support: Provide helpers that convert parsed `Node` values into
  builders for serialize-after-parse pipelines.
- [ ] Stabilization Prep: Audit public types for `#[non_exhaustive]` or
  `#[repr]` as needed and note C FFI considerations.

### Testing

- [ ] Unit tests that cover builder success paths, duplicate keys, unfinished
  structures, and invalid scalars.
- [ ] Golden encoding tests that compare emitted text against expected nosr
  documents, including escape coverage.
- [ ] Round-trip tests that parse sample documents, re-encode, and compare both
  structure and normalized text.
- [ ] Fuzz or property tests for serialization paths once the API settles.
- [ ] Doc tests that exercise the README and module examples.
