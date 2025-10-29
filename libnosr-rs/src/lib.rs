//! # libnosr-rs: Nosr Object Spec Representation Parser
//!
//! A reference implementation of the nosr format parser in Rust.
//!
//! Nosr is a minimal data serialization format that parses into a tree structure
//! consisting of tables (key-value pairs) and vectors (ordered sequences), with
//! scalar values at the leaves. The format is designed to be simple and flexible,
//! allowing you to parse values on-demand rather than converting everything upfront.
//!
//! ## Example
//!
//! ```rust
//! use libnosr_rs::{document, table, text};
//!
//! let source = r#"{
//!     name: "Alice"
//!     age: 30
//! }"#;
//!
//! // Parse the document (lazy - doesn't parse the entire tree yet)
//! let root = document(source).expect("failed to parse document");
//!
//! // Parse the table to get all key-value pairs
//! let tbl = table(&root).expect("failed to parse table");
//!
//! // Access the "name" field from the table
//! let name_node = tbl.get("name").expect("missing name field");
//! let name = text(name_node).expect("name is not a string");
//!
//! assert_eq!(name, "Alice");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod span;

// Re-export the main API types and functions
pub use error::{ParseError, Result};
pub use node::Node;
pub use span::Span;

// Re-export the main API functions
pub use node::{double, table, text, uint64, vector};
pub use parser::document;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_scalar() {
        let source = "hello";
        let node = document(source).expect("failed to parse");
        let value = text(&node).expect("failed to parse as text");
        assert_eq!(value, "hello");
    }
}
