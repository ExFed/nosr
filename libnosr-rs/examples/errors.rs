//! Examples of error handling in nosr parsing.
//!
//! This example demonstrates various error conditions and how to handle them
//! when parsing invalid nosr documents.

use std::collections::HashMap;

use libnosr_rs::error::{ParseError, ParseErrorKind};
use libnosr_rs::{Span, document, double, table, text, uint64, vector};

fn main() {
    println!("=== Nosr Error Handling Examples ===\n");

    // Example 1: Unclosed string
    println!("Example 1: Unclosed string");
    let source = r#""this string is not closed"#;
    let e = document(source).unwrap_err();
    println!("  Error: {}", e);

    // Example 2: Unclosed block comment
    println!("\nExample 2: Unclosed block comment");
    let source = "#* This comment never closes";
    let e = document(source).unwrap_err();
    println!("  Error: {}", e);

    // Example 3: Invalid escape sequence
    println!("\nExample 3: Invalid escape sequence");
    let source = r#""Invalid \x escape""#;
    let node = document(source).expect("document parsing should succeed");
    let e = text(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 4: Type mismatch - trying to parse table as vector
    println!("\nExample 4: Type mismatch (table as vector)");
    let source = "{ key: value }";
    let node = document(source).expect("document parsing should succeed");
    let e = vector(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 5: Type mismatch - trying to parse vector as table
    println!("\nExample 5: Type mismatch (vector as table)");
    let source = "[a, b, c]";
    let node = document(source).expect("document parsing should succeed");
    let e = table(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 6: Type mismatch - trying to parse scalar as table
    println!("\nExample 6: Type mismatch (scalar as table)");
    let source = "just_a_scalar";
    let node = document(source).expect("document parsing should succeed");
    let e = table(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 7: Invalid number format
    println!("\nExample 7: Invalid number format");
    let source = "12.34.56";
    let node = document(source).expect("document parsing should succeed");
    let e = double(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 8: Parsing text as number
    println!("\nExample 8: Parsing text as number");
    let source = "not_a_number";
    let node = document(source).expect("document parsing should succeed");
    let e = uint64(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 9: Integer overflow
    println!("\nExample 9: Integer overflow (max u64 + 1)");
    let source = "18446744073709551616";
    let node = document(source).expect("document parsing should succeed");
    let e = uint64(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 10: Negative number as u64
    println!("\nExample 10: Negative number as u64");
    let source = "-42";
    let node = document(source).expect("document parsing should succeed");
    let e = uint64(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 11: Unbalanced braces
    println!("\nExample 11: Unbalanced braces");
    let source = "{ key: value";
    let node = document(source).expect("document parsing should succeed");
    let e = table(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 12: Unbalanced brackets
    println!("\nExample 12: Unbalanced brackets");
    let source = "[a, b, c";
    let node = document(source).expect("document parsing should succeed");
    let e = vector(&node).unwrap_err();
    println!("  Error: {}", e);

    // Example 13: Error kind inspection
    println!("\nExample 13: Inspecting error kinds");
    let source = r#""unclosed"#;
    if let Err(e) = document(source) {
        println!("  Error occurred at position: {}", e.span.start);
        match e.kind {
            ParseErrorKind::UnclosedString => println!("  Error kind: Unclosed string"),
            ParseErrorKind::UnexpectedEof => println!("  Error kind: Unexpected EOF"),
            _ => println!("  Error kind: {:?}", e.kind),
        }
    }

    // Example 14: Proper error handling in a real scenario
    println!("\nExample 14: Proper error handling pattern");
    let config_source = r#"{
        server: {
            host: localhost
            port: 8080
        }
        database: {
            url: "postgres://localhost/mydb"
        }
    }"#;

    match parse_config(config_source) {
        Ok((host, port)) => println!("  Server config: {}:{}", host, port),
        Err(e) => println!("  Failed to parse config: {}", e),
    }
}

fn parse_config(source: &str) -> Result<(String, u64), ConfigError> {
    let root = document(source)?;
    let config = table(&root)?;

    let server = get_required(&config, "server", root.span())?;
    let server_table = table(server)?;

    let host = get_required(&server_table, "host", server.span())?;
    let host_str = text(host)?.to_string();

    let port = get_required(&server_table, "port", server.span())?;
    let port_num = uint64(port)?;

    Ok((host_str, port_num))
}

// Helper function to get a required key from a table, returning a KeyNotFound error if missing.
fn get_required<'a>(
    table: &'a HashMap<String, libnosr_rs::Node>,
    key: &str,
    span: libnosr_rs::Span,
) -> Result<&'a libnosr_rs::Node<'a>, ConfigError> {
    table
        .get(key)
        .ok_or_else(|| ConfigError::new(ConfigErrorKind::KeyNotFound(key.to_string()), span))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConfigError {
    /// The kind of error that occurred
    pub kind: ConfigErrorKind,
    /// The location in the source where the error occurred
    pub span: Span,
}

impl ConfigError {
    pub fn new(kind: ConfigErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl From<ParseError> for ConfigError {
    fn from(value: ParseError) -> Self {
        Self::new(ConfigErrorKind::Other(value.clone()), value.span)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at position {}", self.kind, self.span.start)
    }
}

impl std::fmt::Display for ConfigErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigErrorKind::KeyNotFound(key) => write!(f, "key not found: {}", key),
            ConfigErrorKind::Other(err) => write!(f, "parse error: {}", err),
        }
    }
}

/// The kind of error that occurred during parsing or navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
enum ConfigErrorKind {
    /// Key not found in table
    KeyNotFound(String),
    /// Other parse error
    Other(ParseError),
}
