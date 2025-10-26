//! Parser for nosr documents.
//!
//! The parser takes a source string and produces a root `Node`.
//! The actual parsing of tables, vectors, and values happens lazily
//! when you navigate the tree.

use crate::error::Result;
use crate::node::Node;
use crate::span::Span;

/// Parse a nosr document from a string.
///
/// This creates a root node representing the entire document.
/// The document is not fully parsed at this point - parsing happens
/// lazily as you navigate the tree.
///
/// # Example
///
/// ```rust
/// use libnosr_rs::document;
///
/// let source = "{ name: Alice }";
/// let root = document(source).expect("failed to parse");
/// ```
pub fn document<'a>(source: &'a str) -> Result<Node<'a>> {
    use crate::lexer::{Lexer, TokenKind};

    // Use the lexer to find the first real token (skipping comments, whitespace, and newlines)
    let mut lexer = Lexer::new(source);

    // Skip newlines and get the first real token
    let mut first_token = lexer.next_token()?;
    while first_token.kind == TokenKind::Newline {
        first_token = lexer.next_token()?;
    }

    if first_token.kind == TokenKind::Eof {
        // Empty document - return empty span
        return Ok(Node::new(source, Span::new(0, 0)));
    }

    // Find the last token to determine the end of the document
    let start_pos = first_token.span.start;
    let mut last_span = first_token.span;

    // Continue through all tokens to find the end
    loop {
        let tok = lexer.next_token()?;
        if tok.kind == TokenKind::Eof {
            break;
        }
        // Only update for non-newline tokens (newlines at end shouldn't count)
        if tok.kind != TokenKind::Newline {
            last_span = tok.span;
        }
    }

    let span = Span::new(start_pos, last_span.end() - start_pos);
    Ok(Node::new(source, span))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::text;

    #[test]
    fn parse_simple_scalar() {
        let source = "hello";
        let node = document(source).unwrap();
        assert_eq!(text(&node).unwrap(), "hello");
    }

    #[test]
    fn parse_quoted_string() {
        let source = r#""hello world""#;
        let node = document(source).unwrap();
        assert_eq!(text(&node).unwrap(), "hello world");
    }

    #[test]
    fn parse_with_whitespace() {
        let source = "  hello  ";
        let node = document(source).unwrap();
        assert_eq!(text(&node).unwrap(), "hello");
    }
}
