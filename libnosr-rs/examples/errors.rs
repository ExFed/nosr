//! Examples of error handling in nosr parsing.
//!
//! This example demonstrates various error conditions and how to handle them
//! when parsing invalid nosr documents.

use libnosr_rs::{document, double, table, text, uint64, vector};
use libnosr_rs::error::{Error, ErrorKind};

fn main() {
    println!("=== Nosr Error Handling Examples ===\n");

    // Example 1: Unclosed string
    println!("Example 1: Unclosed string");
    let source = r#""this string is not closed"#;
    match document(source) {
        Ok(_) => println!("  Unexpectedly succeeded!"),
        Err(e) => println!("  Error: {}", e),
    }

    // Example 2: Unclosed block comment
    println!("\nExample 2: Unclosed block comment");
    let source = "#* This comment never closes";
    match document(source) {
        Ok(_) => println!("  Unexpectedly succeeded!"),
        Err(e) => println!("  Error: {}", e),
    }

    // Example 3: Invalid escape sequence
    println!("\nExample 3: Invalid escape sequence");
    let source = r#""Invalid \x escape""#;
    match document(source) {
        Ok(node) => match text(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 4: Type mismatch - trying to parse table as vector
    println!("\nExample 4: Type mismatch (table as vector)");
    let source = "{ key: value }";
    match document(source) {
        Ok(node) => match vector(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 5: Type mismatch - trying to parse vector as table
    println!("\nExample 5: Type mismatch (vector as table)");
    let source = "[a, b, c]";
    match document(source) {
        Ok(node) => match table(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 6: Type mismatch - trying to parse scalar as table
    println!("\nExample 6: Type mismatch (scalar as table)");
    let source = "just_a_scalar";
    match document(source) {
        Ok(node) => match table(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 7: Invalid number format
    println!("\nExample 7: Invalid number format");
    let source = "12.34.56";
    match document(source) {
        Ok(node) => match double(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 8: Parsing text as number
    println!("\nExample 8: Parsing text as number");
    let source = "not_a_number";
    match document(source) {
        Ok(node) => match uint64(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 9: Integer overflow
    println!("\nExample 9: Integer overflow (max u64 + 1)");
    let source = "18446744073709551616";
    match document(source) {
        Ok(node) => match uint64(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 10: Negative number as u64
    println!("\nExample 10: Negative number as u64");
    let source = "-42";
    match document(source) {
        Ok(node) => match uint64(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 11: Unbalanced braces
    println!("\nExample 11: Unbalanced braces");
    let source = "{ key: value";
    match document(source) {
        Ok(node) => match table(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 12: Unbalanced brackets
    println!("\nExample 12: Unbalanced brackets");
    let source = "[a, b, c";
    match document(source) {
        Ok(node) => match vector(&node) {
            Ok(_) => println!("  Unexpectedly succeeded!"),
            Err(e) => println!("  Error: {}", e),
        },
        Err(e) => println!("  Parse error: {}", e),
    }

    // Example 13: Error kind inspection
    println!("\nExample 13: Inspecting error kinds");
    let source = r#""unclosed"#;
    if let Err(e) = document(source) {
        println!("  Error occurred at position: {}", e.span.start);
        match e.kind {
            ErrorKind::UnclosedString => println!("  Error kind: Unclosed string"),
            ErrorKind::UnexpectedEof => println!("  Error kind: Unexpected EOF"),
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

fn parse_config(source: &str) -> Result<(String, u64), Error> {
    let root = document(source)?;
    let config = table(&root)?;
    
    let server = config.get("server")
        .ok_or_else(|| Error::new(ErrorKind::KeyNotFound("server".to_string()), root.span()))?;
    let server_table = table(server)?;
    
    let host = server_table.get("host")
        .ok_or_else(|| Error::new(ErrorKind::KeyNotFound("host".to_string()), server.span()))?;
    let host_str = text(host)?.to_string();
    
    let port = server_table.get("port")
        .ok_or_else(|| Error::new(ErrorKind::KeyNotFound("port".to_string()), server.span()))?;
    let port_num = uint64(port)?;
    
    Ok((host_str, port_num))
}
