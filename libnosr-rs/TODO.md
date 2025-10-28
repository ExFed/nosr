# libnosr-rs Implementation Plan

## **Overview**

Build `libnosr-rs`, a Rust crate that parses nosr documents into a navigable tree and offers the same high-level API (`document`, `tab`, `vec`, `text`, `uint64`, `double`, etc.) promised by the specification, while staying idiomatic, ergonomic, efficient, and easy to understand as a reference implementation.

## **Requirements**

- Implement the parser in the simplest form that is still correct, keep it well-commented throughout, and document design choices so newcomers can learn from the code.
- Support UTF-8 nosr input, including tables `{}`, vectors `[]`, scalars, quoted text with escapes, whitespace, and both comment styles.
- Represent parsed data as immutable nodes referencing source spans, enabling deferred parsing.
- Provide ergonomic result/option types with precise error context (location, expectation).
- Match the API surface described: lazy document loading, table/vector navigation, scalar conversion helpers.
- Offer optional feature flags for extra conversions (e.g., rationals) without bloating the core.
- Supply documentation and examples reflecting the README rationale.

## **Implementation Steps**

- [x] Initialize crate scaffolding: create `libnosr-rs/`, run `cargo new --lib` (edition 2024), place this plan into `libnosr-rs/TODO.md`, and set up fmt/clippy/test CI placeholders.
- [x] Define the core data model (`Span`, `Source`, `Node` variants, `Result`/`Error` types) with clear comments explaining each structure's purpose; ensure zero-copy string handling via slices or `Cow<'source, str>`.
- [x] Implement a readable lexer: iterate over bytes to emit tokens (braces, brackets, delimiters, string literals with escapes, comments, whitespace); annotate tricky logic with comments and add unit tests for escapes/comments.
- [x] Implement a straightforward recursive-descent parser that transforms tokens into `Node` trees, supporting trailing delimiters, nested tables/vectors, and scalar capture; maintain explanatory comments and docstrings for each parsing function.
- [x] Expose the high-level API:
  - [x] `document` reads from `Read`/`&str`, returning root `Node` or `Error`.
  - [x] `tab`/`vec` perform lazy navigation with clear error messaging when keys/indices are missing.
  - [x] `text`/`uint64`/`double` parse scalars on demand, structuring code for easy extension and documenting parsing assumptions.
- [ ] Provide conversion helpers and traits (e.g., `TryFrom<Node> for u64`, `double` as `f64`) plus optional adapters behind feature flags.
- [x] Add developer-oriented documentation: crate-level docs, module-level comments summarizing parser flow, and examples demonstrating parsing, table lookup, vector iteration, and scalar conversion.
- [ ] Prepare future interop: note potential C FFI hooks and keep public API `#[non_exhaustive]` where appropriate.

## **Testing**

- [x] Lexer unit tests covering strings, escapes, comments, delimiter edge cases, and UTF-8 handling.
- [x] Parser unit tests for tables, vectors, mixed nesting, trailing delimiters, and malformed input producing targeted errors.
- [x] API tests validating `document` + navigation helpers against canonical examples (including README samples).
- [x] Conversion tests for `text`, `uint64`, `double`, including overflow/invalid format cases.
- [ ] Property or fuzz tests feeding random data to ensure resilience and absence of panics.
- [x] Documentation tests (`cargo test --doc`) verifying showcased examples.
- [ ] Optional integration tests comparing outputs against the existing C implementation on shared fixtures once available.

## **Next Steps**

- Add `TryFrom` trait implementations for ergonomic type conversions
- Add more conversion helpers (i8, i16, i32, i64, u8, u16, u32, bool, etc.)
- Consider adding optional features for rationals, decimals, or other extended types
- Add fuzzing tests
- Consider performance optimizations (if needed)
- Add examples directory with more complex use cases
- Consider C FFI bindings for interoperability
