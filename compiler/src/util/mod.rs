use crate::util::error::RaptorexError;
pub mod error;
pub mod types;

// Util structs + types

pub type RaptorexResult<'a, T> = std::result::Result<T, RaptorexError<'a>>;

// Util functions

pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t'
}

pub fn is_whitespace_or_end(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t' || c == ')' || c == '}' || c == '(' || c == '{'
}
