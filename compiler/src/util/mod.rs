pub mod error;

// Util structs + types

#[derive(Debug, Clone, PartialEq)]
pub struct RaptorexError<'a> {
    pub message: String,
    pub line: u32,
    pub file: &'a str,
}

pub type RaptorexResult<'a, T> = std::result::Result<T, RaptorexError<'a>>;

impl std::fmt::Display for RaptorexError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} {}:{}", self.message, self.file, self.line)?;
        Ok(())
    }
}

// Util functions

pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t'
}

pub fn is_whitespace_or_end(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t' || c == ')' || c == '}' || c == '(' || c == '{'
}
