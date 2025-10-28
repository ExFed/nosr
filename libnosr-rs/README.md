# libnosr-rs

A reference implementation of the **Nosr Object Spec Representation** parser in
Rust.

## Overview

Nosr is a minimal data serialization format that parses into a tree structure
consisting of tables (key-value pairs) and vectors (ordered sequences), with
scalar values at the leaves. The format is designed to be simple and flexible,
allowing you to parse values on-demand rather than converting everything
upfront.

This crate provides a well-commented, easy-to-understand implementation that
serves as a reference for the nosr specification.

## Features

- **Lazy parsing**: Only parses what you access
- **Zero-copy where possible**: Uses string slices to avoid allocations
- **Simple and readable**: Clear code with extensive comments
- **Comprehensive error reporting**: Errors include source positions
- **UTF-8 support**: Handles Unicode text correctly
- **Comments**: Supports both line (`#`) and block (`#* ... *#`) comments

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
libnosr-rs = "0.1"
```

## Usage

```rust
use libnosr_rs::{document, table, vector, text, uint64, double};

// Parse a document
let source = r#"{
    name: "Alice"
    age: 30
    scores: [95, 87, 92]
}"#;

let root = document(source)?;

// Navigate tables
let name = table(&root).get("name")?;
assert_eq!(text(&name)?, "Alice");

let age = table(&root).get("age")?;
assert_eq!(uint64(&age)?, 30);

// Navigate vectors
let scores = table(&root).get("scores")?;
let score_vec = vector(&scores)?;
let first_score = &score_vec[0];
assert_eq!(uint64(&first_score)?, 95);
```

## API

The library provides these main functions:

- `document(source: &str) -> Result<Node>` - Parse a nosr document
- `table(node: &Node) -> Result<Map<String, Node>>` - Parse as a table structure
- `vector(node: &Node) -> Result<Vec<Node>>` - Parse as a vector structure
- `text(node: &Node) -> Result<Cow<str>>` - Parse as text
- `uint64(node: &Node) -> Result<u64>` - Parse as unsigned integer
- `double(node: &Node) -> Result<f64>` - Parse as floating-point number

## Examples

### Simple Scalars

```rust
let node = document("hello")?;
assert_eq!(text(&node)?, "hello");

let node = document("42")?;
assert_eq!(uint64(&node)?, 42);
```

### Tables

```rust
let source = "{ name: Alice, age: 30 }";
let root = document(source)?;

let tbl = table(&root)?;
let name = tbl.get("name")?;
assert_eq!(text(&name)?, "Alice");
```

### Vectors

```rust
let source = "[one, two, three]";
let root = document(source)?;

let vec = vector(&root)?;
let first = &vec[0];
assert_eq!(text(&first)?, "one");
```

### Nested Structures

```rust
let source = r#"{
    person: {
        name: Alice
        scores: [95, 87]
    }
}"#;

let root = document(source)?;
let root_tbl = table(&root)?;
let person = root_tbl.get("person")?;
let person_tbl = table(&person)?;
let name = person_tbl.get("name")?;
assert_eq!(text(&name)?, "Alice");

let scores = person_tbl.get("scores")?;
let score_vec = vector(&scores)?;
let first = &score_vec[0];
assert_eq!(uint64(&first)?, 95);
```

## Format Specification

See the [main nosr README](../README.md) for the complete specification.

## Development

```bash
# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
```
