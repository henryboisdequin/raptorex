use std::fmt::*;

pub const KEYWORDS: [&'static str; 19] = [
    "struct", "impl", "fn", "import", "self", "if", "elif", "else", "while", "for", "in", "break",
    "continue", "match", "is", "as", "not", "or", "and",
];

pub const DATA_TYPES: [&'static str; 5] = ["num", "dec", "bool", "Vec", "String"];

/// Tokens in the Raptorex programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // Data types
    String(String),
    Num(i32),
    Dec(f32),
    Bool(bool),
    // Identifiers, e.g. variable, function names, keywords
    Identifier(&'a str),
    // Keywords, e.g. struct, impl, as
    Keyword(&'a str),
    // Data types, usually these tokens reference a return/variable type
    // e.g. foo: **String** = "foo"
    Datatype(&'a str),
    // Operators, e.g +, -, *, /
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Pow,
    // Common syntax, e.g. =>, =, :=, (
    ThinArrow,
    FatArrow,
    Eq,
    EqEq,
    ColonEq,
    EqBangEq,
    Lt,
    Le,
    Gt,
    Ge,
    Dot,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            // Data types
            Token::String(s) => write!(f, "\"{}\"", s)?,
            Token::Num(n) => write!(f, "{}", n)?,
            Token::Dec(d) => write!(f, "{}", d)?,
            Token::Bool(b) => write!(f, "{}", b)?,
            // Identifiers, e.g. variable, function names
            Token::Identifier(i) => write!(f, "{}", i)?,
            // Keywords, e.g. struct, impl, as
            Token::Keyword(k) => write!(f, "{}", k)?,
            // Data types, usually these tokens reference a return/variable type
            // e.g. foo: **String** = "foo"
            Token::Datatype(dt) => write!(f, "{}", dt)?,
            // Operators, e.g +, -, *, /
            Token::Add => write!(f, "+")?,
            Token::Sub => write!(f, "-")?,
            Token::Mul => write!(f, "*")?,
            Token::Div => write!(f, "/")?,
            Token::Modulo => write!(f, "%")?,
            Token::Pow => write!(f, "^")?,
            // Common syntax, e.g. =>, =, :=, (
            Token::ThinArrow => write!(f, "->")?,
            Token::FatArrow => write!(f, "=>")?,
            Token::Eq => write!(f, "=")?,
            Token::EqEq => write!(f, "==")?,
            Token::ColonEq => write!(f, ":=")?,
            Token::EqBangEq => write!(f, "=!=")?,
            Token::Lt => write!(f, "<")?,
            Token::Le => write!(f, "<=")?,
            Token::Gt => write!(f, ">")?,
            Token::Ge => write!(f, ">=")?,
            Token::Dot => write!(f, ".")?,
            Token::Colon => write!(f, ":")?,
            Token::LeftParen => write!(f, "(")?,
            Token::RightParen => write!(f, ")")?,
            Token::LeftBrace => write!(f, "{{")?,
            Token::RightBrace => write!(f, "}}")?,
            Token::LeftBracket => write!(f, "[")?,
            Token::RightBracket => write!(f, "]")?,
            Token::Comma => write!(f, ",")?,
        }

        Ok(())
    }
}
