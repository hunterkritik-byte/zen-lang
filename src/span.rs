#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self { Self { start, end, line, column } }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
}

impl Diagnostic {
    pub fn new(message: impl Into<String>, span: Option<Span>) -> Self { Self { message: message.into(), span } }

    pub fn render(&self, source: &str) -> String {
        let Some(span) = self.span else { return format!("error: {}", self.message) };
        let line_text = source.lines().nth(span.line.saturating_sub(1)).unwrap_or("");
        format!("error: {}\n --> line {}, column {}\n  |\n{:>2} | {}\n  | {}^", self.message, span.line, span.column, span.line, line_text, " ".repeat(span.column.saturating_sub(1)))
    }
}
