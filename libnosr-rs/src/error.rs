//! Error types for nosr parsing.
//!
//! This module provides error types that track where parsing failures occur,
//! helping developers understand what went wrong and where.

use crate::span::Span;
use std::fmt;

/// Result type for nosr operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for nosr parsing and navigation operations.
///
/// All errors include a span indicating where the error occurred,
/// enabling precise error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// The kind of error that occurred
    pub kind: ErrorKind,
    /// The location in the source where the error occurred
    pub span: Span,
}

impl Error {
    /// Create a new error with the given kind and span.
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The kind of error that occurred during parsing or navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
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
    /// Key not found in table
    KeyNotFound(String),
    /// Index out of bounds in vector
    IndexOutOfBounds(usize),
    /// Failed to parse value as requested type
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at position {}", self.kind, self.span.start)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UnexpectedEof => write!(f, "unexpected end of input"),
            ErrorKind::UnexpectedChar(ch) => write!(f, "unexpected character '{}'", ch),
            ErrorKind::ExpectedChar(ch) => write!(f, "expected '{}'", ch),
            ErrorKind::InvalidEscape(ch) => write!(f, "invalid escape sequence '\\{}'", ch),
            ErrorKind::UnclosedString => write!(f, "unclosed string literal"),
            ErrorKind::UnclosedComment => write!(f, "unclosed block comment"),
            ErrorKind::NotATable => write!(f, "expected a table"),
            ErrorKind::NotAVector => write!(f, "expected a vector"),
            ErrorKind::NotAScalar => write!(f, "expected a scalar value"),
            ErrorKind::KeyNotFound(key) => write!(f, "key '{}' not found", key),
            ErrorKind::IndexOutOfBounds(idx) => write!(f, "index {} out of bounds", idx),
            ErrorKind::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = Error::new(ErrorKind::UnexpectedEof, Span::new(42, 0));
        let msg = format!("{}", err);
        assert!(msg.contains("unexpected end of input"));
        assert!(msg.contains("42"));
    }
}
