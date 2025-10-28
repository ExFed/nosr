//! Source span tracking for error reporting.
//!
//! A `Span` represents a contiguous region in the source document,
//! allowing us to report precise error locations and implement zero-copy
//! parsing by referencing slices of the original source.

/// A span representing a region in the source document.
///
/// Tracks the starting byte position and length, enabling zero-copy
/// string extraction and precise error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The starting byte offset in the source
    pub start: usize,
    /// The length in bytes
    pub len: usize,
}

impl Span {
    /// Create a new span from a start position and length.
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    /// Get the end position (exclusive) of this span.
    pub fn end(&self) -> usize {
        self.start + self.len
    }

    /// Extract the substring from the source that this span references.
    pub fn extract<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end()]
    }

    /// Create a span that covers both this span and another.
    ///
    /// Note: This creates a span from the minimum start to maximum end.
    /// If the spans are discontiguous (with a gap between them), the resulting
    /// span will include the content in that gap.
    pub fn merge(&self, other: &Span) -> Span {
        debug_assert!(
            self.end() >= other.start && other.end() >= self.start,
            "Merging discontiguous spans ({}..{} and {}..{}) may produce unexpected results",
            self.start,
            self.end(),
            other.start,
            other.end()
        );
        let start = self.start.min(other.start);
        let end = self.end().max(other.end());
        Span::new(start, end - start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_extract() {
        let source = "hello world";
        let span = Span::new(0, 5);
        assert_eq!(span.extract(source), "hello");

        let span2 = Span::new(6, 5);
        assert_eq!(span2.extract(source), "world");
    }

    #[test]
    fn span_merge() {
        // Test merging overlapping spans
        let span1 = Span::new(5, 5); // 5..10
        let span2 = Span::new(8, 4); // 8..12
        let merged = span1.merge(&span2);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.len, 7); // 5..12
    }
}
