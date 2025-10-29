//! Node types representing parsed nosr values.
//!
//! A `Node` represents a partially-parsed value in a nosr document.
//! Nodes are lazy: they store references to the source text and only
//! parse their contents when you call navigation or conversion functions.
//!
//! This design allows efficient access to deeply nested values without
//! parsing the entire document upfront.

use crate::error::{Error, ErrorKind, Result};
use crate::span::Span;
use std::borrow::Cow;
use std::collections::HashMap;

/// A node in the nosr parse tree.
///
/// Represents a value that may be a table, vector, or scalar.
/// The actual parsing happens lazily when you navigate or convert the node.
#[derive(Debug, Clone)]
pub struct Node<'a> {
    /// The complete source document (needed for extracting substrings)
    source: &'a str,
    /// The span of this node within the source
    span: Span,
}

impl<'a> Node<'a> {
    /// Create a new node referencing a region of the source.
    pub fn new(source: &'a str, span: Span) -> Self {
        Self { source, span }
    }

    /// Get the raw text content of this node (without parsing).
    pub fn raw(&self) -> &'a str {
        self.span.extract(self.source)
    }

    /// Get the span of this node.
    pub fn span(&self) -> Span {
        self.span
    }
}

/// Parse a node as a table.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, table, text};
///
/// let source = "{ name: Alice, age: 30 }";
/// let root = document(source).unwrap();
/// let tbl = table(&root).unwrap();
/// let name = tbl.get("name").unwrap();
/// assert_eq!(text(name).unwrap(), "Alice");
/// ```
pub fn table<'a>(node: &Node<'a>) -> Result<HashMap<String, Node<'a>>> {
    use crate::lexer::{Lexer, TokenKind};

    let content = node.raw().trim();

    // Check if this looks like a table
    if !content.starts_with('{') {
        return Err(Error::new(ErrorKind::NotATable, node.span));
    }

    // Parse the table to collect all key-value pairs
    let mut lexer = Lexer::new(node.source);
    let mut result = HashMap::new();

    // Seek the lexer to our starting position
    lexer.set_pos(node.span.start);

    // Consume the opening brace
    let token = lexer.next_token()?;
    if token.kind != TokenKind::LeftBrace {
        return Err(Error::new(ErrorKind::NotATable, node.span));
    }

    // Parse key-value pairs
    loop {
        // Skip delimiters, but detect consecutive commas without intervening newlines
        let mut tok = lexer.next_token()?;
        let mut saw_comma = false;
        while matches!(tok.kind, TokenKind::Newline | TokenKind::Comma) {
            if tok.kind == TokenKind::Comma {
                if saw_comma {
                    // Multiple consecutive commas without intervening newlines or elements
                    return Err(Error::new(ErrorKind::ConsecutiveDelimiters, tok.span));
                }
                saw_comma = true;
            } else {
                // Newline resets the comma tracking
                saw_comma = false;
            }
            tok = lexer.next_token()?;
        }

        // Check for end of table
        if tok.kind == TokenKind::RightBrace {
            break;
        }

        // Get the key (should be a string or scalar)
        let key_span = tok.span;
        let key_text = if tok.kind == TokenKind::String {
            // Parse as quoted string
            let key_node = Node::new(node.source, key_span);
            text(&key_node)?.into_owned()
        } else if tok.kind == TokenKind::Scalar {
            key_span.extract(node.source).to_string()
        } else {
            return Err(Error::new(ErrorKind::ExpectedChar(':'), tok.span));
        };

        // Expect a colon
        tok = lexer.next_token()?;
        if tok.kind != TokenKind::Colon {
            return Err(Error::new(ErrorKind::ExpectedChar(':'), tok.span));
        }

        // Get the value
        tok = lexer.next_token()?;
        let value_start = tok.span;

        // Determine value extent (might be a nested structure)
        let value_end = match tok.kind {
            TokenKind::LeftBrace => {
                // Parse nested table
                parse_balanced(node.source, &mut lexer, TokenKind::RightBrace)?
            }
            TokenKind::LeftBracket => {
                // Parse nested vector
                parse_balanced(node.source, &mut lexer, TokenKind::RightBracket)?
            }
            TokenKind::String | TokenKind::Scalar => {
                // Simple value
                tok.span
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedChar(
                        value_start
                            .extract(node.source)
                            .chars()
                            .next()
                            .unwrap_or(' '),
                    ),
                    value_start,
                ));
            }
        };

        // Add to the result map
        let value_span = Span::new(value_start.start, value_end.end() - value_start.start);
        result.insert(key_text, Node::new(node.source, value_span));
    }

    Ok(result)
}

/// Helper function to parse balanced braces/brackets.
///
/// Returns the span of the closing delimiter.
fn parse_balanced(
    _source: &str,
    lexer: &mut crate::lexer::Lexer,
    closing: crate::lexer::TokenKind,
) -> Result<Span> {
    use crate::lexer::TokenKind;

    let mut depth = 1;
    let mut last_span = Span::new(0, 0);

    while depth > 0 {
        let tok = lexer.next_token()?;
        last_span = tok.span;

        match tok.kind {
            TokenKind::LeftBrace | TokenKind::LeftBracket => {
                depth += 1;
            }
            TokenKind::RightBrace | TokenKind::RightBracket => {
                depth -= 1;
                if depth == 0 && tok.kind == closing {
                    return Ok(tok.span);
                }
            }
            TokenKind::Eof => {
                return Err(Error::new(ErrorKind::UnexpectedEof, tok.span));
            }
            _ => {}
        }
    }

    Ok(last_span)
}

/// Parse a node as a vector and return all elements.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, vector, text};
///
/// let source = "[hello, world]";
/// let root = document(source).unwrap();
/// let v = vector(&root).unwrap();
/// assert_eq!(text(&v[0]).unwrap(), "hello");
/// assert_eq!(text(&v[1]).unwrap(), "world");
/// ```
pub fn vector<'a>(node: &Node<'a>) -> Result<Vec<Node<'a>>> {
    use crate::lexer::{Lexer, TokenKind};

    let content = node.raw().trim();

    // Check if this looks like a vector
    if !content.starts_with('[') {
        return Err(Error::new(ErrorKind::NotAVector, node.span));
    }

    // Parse the vector to collect all elements
    let mut lexer = Lexer::new(node.source);
    let mut result = Vec::new();

    // Seek the lexer to our starting position
    lexer.set_pos(node.span.start);

    // Consume the opening bracket
    let token = lexer.next_token()?;
    if token.kind != TokenKind::LeftBracket {
        return Err(Error::new(ErrorKind::NotAVector, node.span));
    }

    // Parse elements
    loop {
        // Skip delimiters, but detect consecutive commas without intervening newlines
        let mut tok = lexer.next_token()?;
        let mut saw_comma = false;
        while matches!(tok.kind, TokenKind::Newline | TokenKind::Comma) {
            if tok.kind == TokenKind::Comma {
                if saw_comma {
                    // Multiple consecutive commas without intervening newlines or elements
                    return Err(Error::new(ErrorKind::ConsecutiveDelimiters, tok.span));
                }
                saw_comma = true;
            } else {
                // Newline resets the comma tracking
                saw_comma = false;
            }
            tok = lexer.next_token()?;
        }

        // Check for end of vector
        if tok.kind == TokenKind::RightBracket {
            break;
        }

        // Get the element
        let elem_start = tok.span;

        // Determine element extent (might be a nested structure)
        let elem_end = match tok.kind {
            TokenKind::LeftBrace => {
                // Parse nested table
                parse_balanced(node.source, &mut lexer, TokenKind::RightBrace)?
            }
            TokenKind::LeftBracket => {
                // Parse nested vector
                parse_balanced(node.source, &mut lexer, TokenKind::RightBracket)?
            }
            TokenKind::String | TokenKind::Scalar => {
                // Simple value
                tok.span
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedChar(
                        elem_start
                            .extract(node.source)
                            .chars()
                            .next()
                            .unwrap_or(' '),
                    ),
                    elem_start,
                ));
            }
        };

        // Add to the result vector
        let elem_span = Span::new(elem_start.start, elem_end.end() - elem_start.start);
        result.push(Node::new(node.source, elem_span));
    }

    Ok(result)
}

/// Parse a node as a text string.
///
/// Handles both quoted strings (with escape sequences) and raw scalars.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, text};
///
/// let node = document("\"hello world\"").unwrap();
/// assert_eq!(text(&node).unwrap(), "hello world");
///
/// let node2 = document("hello").unwrap();
/// assert_eq!(text(&node2).unwrap(), "hello");
/// ```
pub fn text<'a>(node: &Node<'a>) -> Result<Cow<'a, str>> {
    let content = node.raw().trim();

    if content.is_empty() {
        return Err(Error::new(ErrorKind::NotAScalar, node.span));
    }

    // Check if it's a quoted string
    if content.starts_with('"') {
        if !content.ends_with('"') || content.len() < 2 {
            return Err(Error::new(ErrorKind::UnclosedString, node.span));
        }

        // Extract the content between quotes
        let inner = &content[1..content.len() - 1];

        // Process the string, only allocating if we find escape sequences
        let mut result = None;
        let mut chars = inner.chars();
        let mut pos = 0;

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                // Found an escape - initialize result if needed
                let s = result.get_or_insert_with(|| String::from(&inner[..pos]));

                match chars.next() {
                    Some('\\') => s.push('\\'),
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some(':') => s.push(':'),
                    Some('"') => s.push('"'),
                    Some('[') => s.push('['),
                    Some(']') => s.push(']'),
                    Some('{') => s.push('{'),
                    Some('}') => s.push('}'),
                    Some(other) => {
                        return Err(Error::new(ErrorKind::InvalidEscape(other), node.span));
                    }
                    None => return Err(Error::new(ErrorKind::UnexpectedEof, node.span)),
                }
                pos += 2; // backslash + escaped char
            } else {
                if let Some(ref mut s) = result {
                    s.push(ch);
                }
                pos += ch.len_utf8();
            }
        }

        // Return owned if we found escapes, borrowed otherwise
        Ok(result.map(Cow::Owned).unwrap_or(Cow::Borrowed(inner)))
    } else {
        // Unquoted scalar - return as-is
        Ok(Cow::Borrowed(content))
    }
}

/// Parse a node as a 64-bit unsigned integer.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, uint64};
///
/// let node = document("12345").unwrap();
/// assert_eq!(uint64(&node).unwrap(), 12345);
/// ```
pub fn uint64(node: &Node) -> Result<u64> {
    let content = node.raw().trim();

    content.parse::<u64>().map_err(|e| {
        Error::new(
            ErrorKind::ParseError(format!("failed to parse as u64: {}", e)),
            node.span,
        )
    })
}

/// Parse a node as a double-precision floating-point number.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, double};
///
/// let node = document("3.14159").unwrap();
/// assert!((double(&node).unwrap() - 3.14159).abs() < 0.00001);
/// ```
pub fn double(node: &Node) -> Result<f64> {
    let content = node.raw().trim();

    content.parse::<f64>().map_err(|e| {
        Error::new(
            ErrorKind::ParseError(format!("failed to parse as f64: {}", e)),
            node.span,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_extraction() {
        let source = r#"{
        person: {
            name: Alice
        }
    }"#;
        let root = Node::new(source, Span::new(0, source.len()));
        let tbl = table(&root).expect("failed to parse table");
        let person = tbl.get("person").expect("person key not found");

        // Check what we extracted
        let raw = person.raw();
        eprintln!("Extracted: {:?}", raw);
        assert!(
            raw.trim().starts_with('{'),
            "Should start with brace, got: {}",
            raw
        );
    }

    #[test]
    fn text_unquoted() {
        let source = "hello";
        let node = Node::new(source, Span::new(0, 5));
        assert_eq!(text(&node).unwrap(), "hello");
    }

    #[test]
    fn text_quoted() {
        let source = r#""hello world""#;
        let node = Node::new(source, Span::new(0, source.len()));
        assert_eq!(text(&node).unwrap(), "hello world");
    }

    #[test]
    fn text_with_escapes() {
        let source = r#""hello\nworld""#;
        let node = Node::new(source, Span::new(0, source.len()));
        assert_eq!(text(&node).unwrap(), "hello\nworld");
    }

    #[test]
    fn parse_uint64() {
        let source = "42";
        let node = Node::new(source, Span::new(0, 2));
        assert_eq!(uint64(&node).unwrap(), 42);
    }

    #[test]
    fn parse_double() {
        let source = "12.5";
        let node = Node::new(source, Span::new(0, 4));
        assert!((double(&node).unwrap() - 12.5).abs() < 0.0001);
    }
}
