#[derive(Clone, PartialEq, Debug)]
pub enum Types<'a> {
    Scope,
    Num(i32),
    Dec(f32),
    Bool(bool),
    Vec(Vec<Types<'a>>),
    String(String),
    FunctionCall(&'a str),
    Operator(Operator),
    CommonSyntax(CommonSyntax),
    Keyword(&'a str),
    Identifier(&'a str),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CommonSyntax {
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

pub const KEYWORDS: [&'static str; 19] = [
    "struct", "impl", "fn", "import", "self", "if", "elif", "else", "while", "for", "in", "break",
    "continue", "match", "is", "as", "not", "or", "and",
];

pub const DATA_TYPES: [&'static str; 5] = ["num", "dec", "bool", "Vec", "String"];
