#[derive(Debug, Clone)]
pub struct RaptorexError<'a> {
    pub message: &'a str,
    pub line: usize,
    pub file: &'a str,
}

impl<'a> RaptorexError<'a> {
    pub fn new() -> Self {
        Self {
            message: "",
            line: 0,
            file: "",
        }
    }
    pub fn throw_error(message: &'a str, line: usize, file: &'a str) -> Self {
        RaptorexError {
            message,
            line,
            file,
        }
    }
}
