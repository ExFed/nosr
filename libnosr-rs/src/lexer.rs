//! Lexical analysis for nosr documents.
//!
//! The lexer breaks the input into tokens, handling:
//! - Structural characters: `{`, `}`, `[`, `]`, `:`, `,`
//! - String literals with escape sequences
//! - Comments (line and block)
//! - Whitespace
//! - Scalar values (everything else)
//!
//! The lexer is designed to be simple and easy to understand, serving
//! as a reference implementation of the nosr specification.

use crate::error::{Error, ErrorKind, Result};
use crate::span::Span;

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// The location of this token in the source
    pub span: Span,
}

/// The kind of token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Opening brace `{`
    LeftBrace,
    /// Closing brace `}`
    RightBrace,
    /// Opening bracket `[`
    LeftBracket,
    /// Closing bracket `]`
    RightBracket,
    /// Colon `:`
    Colon,
    /// Comma `,`
    Comma,
    /// Newline (serves as delimiter)
    Newline,
    /// A string literal (contents stored in source via span)
    String,
    /// A scalar value (unquoted text)
    Scalar,
    /// End of input
    Eof,
}

/// A lexer that tokenizes nosr input.
///
/// The lexer maintains its position in the source and provides
/// methods to peek at and consume tokens.
pub struct Lexer<'a> {
    /// The source text being lexed
    source: &'a str,
    /// Current byte position in the source
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source.
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    /// Get the current position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Set the lexer position (for seeking to a specific location).
    ///
    /// # Safety
    /// The position must be at a valid UTF-8 character boundary.
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Peek at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// Peek at the character at a given offset from current position.
    fn peek_at(&self, offset: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(offset)
    }

    /// Consume and return the current character.
    fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    /// Skip whitespace (but not newlines, which are significant).
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.consume();
            } else {
                break;
            }
        }
    }

    /// Skip a line comment (from `#` to end of line).
    fn skip_line_comment(&mut self) {
        // Consume the #
        self.consume();

        // Skip until newline
        while let Some(ch) = self.peek() {
            self.consume();
            if ch == '\n' {
                break;
            }
        }
    }

    /// Skip a block comment (from `#*` to `*#`).
    fn skip_block_comment(&mut self) -> Result<()> {
        let start = self.pos;

        // Consume the #*
        self.consume();
        self.consume();

        // Find the closing *#
        loop {
            match self.peek() {
                None => {
                    return Err(Error::new(
                        ErrorKind::UnclosedComment,
                        Span::new(start, self.pos - start),
                    ));
                }
                Some('*') => {
                    self.consume();
                    if let Some('#') = self.peek() {
                        self.consume();
                        return Ok(());
                    }
                }
                Some(_) => {
                    self.consume();
                }
            }
        }
    }

    /// Lex a string literal (from `"` to `"`).
    ///
    /// Handles escape sequences within the string.
    fn lex_string(&mut self) -> Result<Token> {
        let start = self.pos;

        // Consume opening quote
        self.consume();

        // Find closing quote, handling escapes
        loop {
            match self.peek() {
                None => {
                    return Err(Error::new(
                        ErrorKind::UnclosedString,
                        Span::new(start, self.pos - start),
                    ));
                }
                Some('"') => {
                    self.consume();
                    break;
                }
                Some('\\') => {
                    // Consume the backslash
                    self.consume();
                    // Consume the escaped character (validation happens in text())
                    if self.consume().is_none() {
                        return Err(Error::new(
                            ErrorKind::UnclosedString,
                            Span::new(start, self.pos - start),
                        ));
                    }
                }
                Some(_) => {
                    self.consume();
                }
            }
        }

        Ok(Token {
            kind: TokenKind::String,
            span: Span::new(start, self.pos - start),
        })
    }

    /// Lex a scalar (unquoted text).
    ///
    /// Continues until we hit whitespace or a structural character.
    fn lex_scalar(&mut self) -> Token {
        let start = self.pos;

        while let Some(ch) = self.peek() {
            // Stop at structural characters or whitespace
            if matches!(
                ch,
                '{' | '}' | '[' | ']' | ':' | ',' | '\n' | ' ' | '\t' | '\r'
            ) {
                break;
            }

            // Stop at comment starts
            if ch == '#' && matches!(self.peek_at(1), Some('*')) {
                break;
            }

            self.consume();
        }

        Token {
            kind: TokenKind::Scalar,
            span: Span::new(start, self.pos - start),
        }
    }

    /// Get the next token.
    pub fn next_token(&mut self) -> Result<Token> {
        loop {
            // Skip non-newline whitespace
            self.skip_whitespace();

            let start = self.pos;

            match self.peek() {
                None => {
                    return Ok(Token {
                        kind: TokenKind::Eof,
                        span: Span::new(start, 0),
                    });
                }
                Some('\n') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::Newline,
                        span: Span::new(start, 1),
                    });
                }
                Some('{') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::LeftBrace,
                        span: Span::new(start, 1),
                    });
                }
                Some('}') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::RightBrace,
                        span: Span::new(start, 1),
                    });
                }
                Some('[') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::LeftBracket,
                        span: Span::new(start, 1),
                    });
                }
                Some(']') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::RightBracket,
                        span: Span::new(start, 1),
                    });
                }
                Some(':') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::Colon,
                        span: Span::new(start, 1),
                    });
                }
                Some(',') => {
                    self.consume();
                    return Ok(Token {
                        kind: TokenKind::Comma,
                        span: Span::new(start, 1),
                    });
                }
                Some('"') => {
                    return self.lex_string();
                }
                Some('#') => {
                    // Check for comments
                    match self.peek_at(1) {
                        Some('*') => {
                            self.skip_block_comment()?;
                            continue; // Loop to get next token
                        }
                        _ => {
                            // Line comment - skip until newline
                            self.skip_line_comment();
                            continue; // Loop to get next token
                        }
                    }
                }
                Some(_) => {
                    return Ok(self.lex_scalar());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_braces() {
        let mut lexer = Lexer::new("{ }");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::RightBrace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn lex_brackets() {
        let mut lexer = Lexer::new("[ ]");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LeftBracket);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::RightBracket);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn lex_string() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.span.extract(r#""hello world""#), r#""hello world""#);
    }

    #[test]
    fn lex_scalar() {
        let mut lexer = Lexer::new("hello");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Scalar);
        assert_eq!(token.span.extract("hello"), "hello");
    }

    #[test]
    fn lex_line_comment() {
        let mut lexer = Lexer::new("# comment\nhello");
        // Line comment consumes until and including the newline
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Scalar);
        assert_eq!(token.span.extract("# comment\nhello"), "hello");
    }

    #[test]
    fn lex_block_comment() {
        let mut lexer = Lexer::new("#* comment *# hello");
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Scalar);
        assert_eq!(token.span.extract("#* comment *# hello"), "hello");
    }

    #[test]
    fn lex_unclosed_comment() {
        let mut lexer = Lexer::new("#* unclosed");
        let result = lexer.next_token();
        assert!(matches!(
            result,
            Err(Error {
                kind: ErrorKind::UnclosedComment,
                ..
            })
        ));
    }

    #[test]
    fn lex_table() {
        let mut lexer = Lexer::new("{ key: value }");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Scalar);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Colon);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Scalar);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::RightBrace);
    }
}
