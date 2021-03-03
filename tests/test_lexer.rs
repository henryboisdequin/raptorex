#[cfg(test)]
mod test_lexer {
    use compiler::lexer::{lexer::Lexer, tokens::Token};

    #[test]
    fn create_string() {
        let mut lexer = Lexer::new("\"Hello world!\"");
        assert_eq!(
            lexer.tokens(),
            vec![Token::String("Hello world!".to_string())]
        );
    }

    #[test]
    fn create_normal_fn() {
        let mut lexer = Lexer::new("fn main() -> [] {\n[]\n}");
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Keyword("fn"),
                Token::Identifier("main"),
                Token::LeftParen,
                Token::RightParen,
                Token::ThinArrow,
                Token::LeftBracket,
                Token::RightBracket,
                Token::LeftBrace,
                Token::LeftBracket,
                Token::RightBracket,
                Token::RightBrace,
            ]
        );
    }

    #[test]
    fn create_normal_program() {
        let mut lexer = Lexer::new("fn main() -> num {\nfoobar: num = 1;\n=> foobar;\n}");
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Keyword("fn"),
                Token::Identifier("main"),
                Token::LeftParen,
                Token::RightParen,
                Token::ThinArrow,
                Token::Datatype("num"),
                Token::LeftBrace,
                Token::Identifier("foobar"),
                Token::Colon,
                Token::Datatype("num"),
                Token::Eq,
                Token::Num(1),
                Token::SemiColon,
                Token::FatArrow,
                Token::Identifier("foobar"),
                Token::SemiColon,
                Token::RightBrace,
            ]
        );
    }

    #[test]
    fn create_number() {
        let mut lexer = Lexer::new("1 2 4");
        assert_eq!(
            lexer.tokens(),
            vec![Token::Num(1), Token::Num(2), Token::Num(4)]
        );
    }

    #[test]
    fn create_decimal() {
        let mut lexer = Lexer::new("3.14 23.1");
        assert_eq!(lexer.tokens(), vec![Token::Dec(3.14), Token::Dec(23.1)]);
    }

    #[test]
    fn create_boolean() {
        let mut lexer = Lexer::new("true true false");
        assert_eq!(
            lexer.tokens(),
            vec![Token::Bool(true), Token::Bool(true), Token::Bool(false)]
        );
    }

    #[test]
    fn create_parenthesis() {
        let mut lexer = Lexer::new("())");
        assert_eq!(
            lexer.tokens(),
            vec![Token::LeftParen, Token::RightParen, Token::RightParen]
        );
    }

    #[test]
    fn create_equals() {
        let mut lexer = Lexer::new(":= = =>");
        assert_eq!(
            lexer.tokens(),
            vec![Token::ColonEq, Token::Eq, Token::FatArrow]
        );
    }

    #[test]
    fn common_syntax() {
        let mut lexer = Lexer::new(", = -> =!= >;");
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Comma,
                Token::Eq,
                Token::ThinArrow,
                Token::EqBangEq,
                Token::Gt,
                Token::SemiColon,
            ]
        );
    }

    #[test]
    fn create_comment() {
        let mut lexer = Lexer::new("# this is a comment\n1");
        assert_eq!(lexer.tokens(), vec![Token::Num(1)]);
    }

    #[test]
    fn create_operations() {
        let mut lexer = Lexer::new("1 + 2 / 4 % 2 * 3 ^ 101");
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Num(1),
                Token::Add,
                Token::Num(2),
                Token::Div,
                Token::Num(4),
                Token::Modulo,
                Token::Num(2),
                Token::Mul,
                Token::Num(3),
                Token::Pow,
                Token::Num(101)
            ]
        );
    }

    #[test]
    fn create_data_types() {
        let mut lexer = Lexer::new("bool num String Vec dec");
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Datatype("bool"),
                Token::Datatype("num"),
                Token::Datatype("String"),
                Token::Datatype("Vec"),
                Token::Datatype("dec")
            ]
        );
    }
}
