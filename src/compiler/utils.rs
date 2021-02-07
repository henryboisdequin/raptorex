pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t'
}

pub fn is_whitespace_or_end(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t' || c == ')' || c == '}' || c == '(' || c == '{'
}
