//! Error types for nosr parsing.
//!
//! This module provides error types that track where parsing failures occur,
//! helping developers understand what went wrong and where.

use crate::span::Span;
use std::fmt;

/// Result type for nosr operations.
pub type Result<T> = std::result::Result<T, ParseError>;

/// Error type for nosr parsing and navigation operations.
///
/// All errors include a span indicating where the error occurred,
/// enabling precise error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The kind of error that occurred
    pub kind: ParseErrorKind,
    /// The location in the source where the error occurred
    pub span: Span,
}

impl ParseError {
    /// Create a new error with the given kind and span.
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The kind of error that occurred during parsing or navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// Unexpected end of input
    UnexpectedEof,
    /// Unexpected character encountered
    UnexpectedChar(char),
    /// Expected a specific character but found something else
    ExpectedChar(char),
    /// Invalid escape sequence in a string
    InvalidEscape(char),
    /// Unclosed string literal
    UnclosedString,
    /// Unclosed block comment
    UnclosedComment,
    /// Expected a table but found something else
    NotATable,
    /// Expected a vector but found something else
    NotAVector,
    /// Expected a scalar value but found something else
    NotAScalar,
    /// Failed to parse value as requested type
    ParseError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at position {}", self.kind, self.span.start)
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseErrorKind::UnexpectedChar(ch) => write!(f, "unexpected character '{}'", ch),
            ParseErrorKind::ExpectedChar(ch) => write!(f, "expected '{}'", ch),
            ParseErrorKind::InvalidEscape(ch) => write!(f, "invalid escape sequence '\\{}'", ch),
            ParseErrorKind::UnclosedString => write!(f, "unclosed string literal"),
            ParseErrorKind::UnclosedComment => write!(f, "unclosed block comment"),
            ParseErrorKind::NotATable => write!(f, "expected a table"),
            ParseErrorKind::NotAVector => write!(f, "expected a vector"),
            ParseErrorKind::NotAScalar => write!(f, "expected a scalar value"),
            ParseErrorKind::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = ParseError::new(ParseErrorKind::UnexpectedEof, Span::new(42, 0));
        let msg = format!("{}", err);
        assert!(msg.contains("unexpected end of input"));
        assert!(msg.contains("42"));
    }
}
