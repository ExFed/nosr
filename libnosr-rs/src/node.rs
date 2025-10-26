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

/// Navigate to a key in a table node.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, tab, text};
///
/// let source = "{ name: Alice }";
/// let root = document(source).unwrap();
/// let name = tab(&root, "name").unwrap();
/// assert_eq!(text(&name).unwrap(), "Alice");
/// ```
pub fn tab<'a>(node: &Node<'a>, key: &str) -> Result<Node<'a>> {
    use crate::lexer::{Lexer, TokenKind};

    let content = node.raw().trim();

    // Check if this looks like a table
    if !content.starts_with('{') {
        return Err(Error::new(ErrorKind::NotATable, node.span));
    }

    // Parse the table to find the key
    let mut lexer = Lexer::new(node.source);

    // Seek the lexer to our starting position
    lexer.set_pos(node.span.start);

    // Consume the opening brace
    let token = lexer.next_token()?;
    if token.kind != TokenKind::LeftBrace {
        return Err(Error::new(ErrorKind::NotATable, node.span));
    }

    // Parse key-value pairs
    loop {
        // Skip delimiters (newlines, commas, semicolons)
        let mut tok = lexer.next_token()?;
        if matches!(
            tok.kind,
            TokenKind::Newline | TokenKind::Comma | TokenKind::Semicolon
        ) {
            continue;
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

        // Check if this is the key we're looking for
        if key_text == key {
            let value_span = Span::new(value_start.start, value_end.end() - value_start.start);
            return Ok(Node::new(node.source, value_span));
        }
    }

    Err(Error::new(
        ErrorKind::KeyNotFound(key.to_string()),
        node.span,
    ))
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

/// Navigate to an index in a vector node.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::{document, vec, text};
///
/// let source = "[hello, world]";
/// let root = document(source).unwrap();
/// let first = vec(&root, 0).unwrap();
/// assert_eq!(text(&first).unwrap(), "hello");
/// ```
pub fn vec<'a>(node: &Node<'a>, index: usize) -> Result<Node<'a>> {
    use crate::lexer::{Lexer, TokenKind};

    let content = node.raw().trim();

    // Check if this looks like a vector
    if !content.starts_with('[') {
        return Err(Error::new(ErrorKind::NotAVector, node.span));
    }

    // Parse the vector to find the nth element
    let mut lexer = Lexer::new(node.source);

    // Seek the lexer to our starting position
    lexer.set_pos(node.span.start);

    // Consume the opening bracket
    let token = lexer.next_token()?;
    if token.kind != TokenKind::LeftBracket {
        return Err(Error::new(ErrorKind::NotAVector, node.span));
    }

    // Parse elements
    let mut current_index = 0;

    loop {
        // Skip delimiters (newlines, commas, semicolons)
        let mut tok = lexer.next_token()?;
        while matches!(
            tok.kind,
            TokenKind::Newline | TokenKind::Comma | TokenKind::Semicolon
        ) {
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

        // Check if this is the index we're looking for
        if current_index == index {
            let elem_span = Span::new(elem_start.start, elem_end.end() - elem_start.start);
            return Ok(Node::new(node.source, elem_span));
        }

        current_index += 1;
    }

    Err(Error::new(ErrorKind::IndexOutOfBounds(index), node.span))
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
        let person = tab(&root, "person").expect("failed to get person");

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
        let source = "3.14";
        let node = Node::new(source, Span::new(0, 4));
        assert!((double(&node).unwrap() - 3.14).abs() < 0.0001);
    }
}
