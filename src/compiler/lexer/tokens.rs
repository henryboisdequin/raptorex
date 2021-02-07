use std::fmt::*;

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
    // Operators, e.g +, -, *, /
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Pow,
    // Keywords, e.g. fn, struct, etc
    // Struct,
    // Impl,
    // Import,
    // Fn,
    // This,
    // If,
    // Else,
    // Elif,
    // While,
    // For,
    // In,
    // Break,
    // Continue,
    // Match,
    // Is,
    // As,
    // Not,
    // Or,
    // And,
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
            // Operators, e.g +, -, *, /
            Token::Add => write!(f, "+")?,
            Token::Sub => write!(f, "-")?,
            Token::Mul => write!(f, "*")?,
            Token::Div => write!(f, "/")?,
            Token::Modulo => write!(f, "%")?,
            Token::Pow => write!(f, "^")?,
            // Keywords, e.g. fn, struct, etc
            // Token::Struct => write!(f, "struct")?,
            // Token::Impl => write!(f, "impl")?,
            // Token::Import => write!(f, "import")?,
            // Token::Fn => write!(f, "fn")?,
            // Token::This => write!(f, "self")?,
            // Token::If => write!(f, "if")?,
            // Token::Else => write!(f, "else")?,
            // Token::Elif => write!(f, "elif")?,
            // Token::While => write!(f, "while")?,
            // Token::For => write!(f, "for")?,
            // Token::In => write!(f, "in")?,
            // Token::Break => write!(f, "break")?,
            // Token::Continue => write!(f, "continue")?,
            // Token::Match => write!(f, "match")?,
            // Token::Is => write!(f, "is")?,
            // Token::As => write!(f, "as")?,
            // Token::Not => write!(f, "not")?,
            // Token::Or => write!(f, "or")?,
            // Token::And => write!(f, "and")?,
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
