use crate::lexer::tokens::Token;
use crate::util::types::{DATA_TYPES, KEYWORDS};
use crate::util::*;

pub struct Lexer<'a> {
    code: &'a str,
    tokens: Vec<Token<'a>>,
    current_pos: usize,
    start_pos: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            tokens: Vec::new(),
            current_pos: 0,
            start_pos: 0,
            line: 0,
        }
    }

    fn next_char(&mut self) -> char {
        self.code.chars().nth(self.current_pos).unwrap_or('\0')
    }

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        self.current_pos += 1;
        self.code.chars().nth(self.current_pos - 1).unwrap_or('\0')
    }

    fn is_end(&mut self) -> bool {
        self.current_pos >= self.code.len()
    }

    fn make_tokens(&mut self) {
        match self.advance() {
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            '[' => self.add_token(Token::LeftBracket),
            ']' => self.add_token(Token::RightBracket),
            ';' => self.add_token(Token::SemiColon),
            '"' => self.make_string(),
            '#' => self.skip_comment(),
            '=' => self.make_equals(),
            '+' => self.add_token(Token::Add),
            '-' => self.make_minus_or_thin_arrow(),
            '*' => self.add_token(Token::Mul),
            '/' => self.add_token(Token::Div),
            '^' => self.add_token(Token::Pow),
            '%' => self.add_token(Token::Modulo),
            '<' => self.make_less_than(),
            '>' => self.make_greater_than(),
            ',' => self.add_token(Token::Comma),
            ':' => self.make_colon(),
            x => {
                if x.is_digit(10) {
                    self.make_number();
                } else {
                    self.add_other();
                }
            }
        }
    }

    fn make_string(&mut self) {
        let mut string = String::new();

        while !self.is_end() && self.next_char() != '"' {
            if self.next_char() == '\n' {
                self.line += 1;
            } else {
                string += self.next_char().encode_utf8(&mut [0; 4]);
            }
            self.advance();
        }
        self.advance();

        self.tokens.push(Token::String(string));
    }

    fn make_number(&mut self) {
        let mut dot_count = 0;

        while self.next_char().is_ascii_alphanumeric() && !self.is_end() {
            self.advance();
        }

        if self.next_char() == '.' {
            dot_count += 1;
            self.advance();
        }

        while self.next_char().is_ascii_alphanumeric() && !self.is_end() {
            self.advance();
        }

        let final_num = &self.code[self.start_pos..self.current_pos];

        if dot_count != 0 {
            self.add_token(Token::Dec(final_num.parse::<f32>().unwrap()));
        } else {
            self.add_token(Token::Num(final_num.parse::<i32>().unwrap()));
        }
    }

    fn make_less_than(&mut self) {
        let mut is_less_than = true;
        self.advance();

        if self.next_char() == '=' {
            is_less_than = false;
            self.advance();
        }

        if is_less_than {
            self.add_token(Token::Lt);
        } else {
            self.add_token(Token::Le);
        }
    }

    fn make_greater_than(&mut self) {
        let mut is_greater_than = true;
        self.advance();

        if self.next_char() == '=' {
            is_greater_than = false;
            self.advance();
        }

        if is_greater_than {
            self.add_token(Token::Gt);
        } else {
            self.add_token(Token::Ge);
        }
    }

    fn make_minus_or_thin_arrow(&mut self) {
        let mut is_minus = true;

        if self.advance() == '>' {
            is_minus = false;
            self.advance();
        }

        if is_minus {
            self.add_token(Token::Sub);
        } else {
            self.add_token(Token::ThinArrow);
        }
    }

    fn make_equals(&mut self) {
        let mut is_equal_equal = false;
        let mut is_bang_equal = false;
        let mut is_fat_arrow = false;
        let mut current_char = self.advance();

        if current_char == '=' {
            is_equal_equal == true;
            current_char = self.advance();
        } else if current_char == '!' {
            current_char = self.advance();
            if current_char == '=' {
                is_bang_equal = true;
                current_char = self.advance();
            }
        } else if current_char == '>' {
            is_fat_arrow = true;
            current_char = self.advance();
        }

        if is_equal_equal {
            self.add_token(Token::EqEq);
        } else if is_bang_equal {
            self.add_token(Token::EqBangEq);
        } else if is_fat_arrow {
            self.add_token(Token::FatArrow);
        } else {
            self.add_token(Token::Eq);
        }
    }

    fn make_colon(&mut self) {
        let mut is_colon_equals = false;
        let mut current_char = self.advance();

        if current_char == '=' {
            is_colon_equals = true;
            current_char = self.advance();
        }

        if is_colon_equals {
            self.add_token(Token::ColonEq);
        } else {
            self.add_token(Token::Colon);
        }
    }

    fn add_other(&mut self) {
        let mut colon = false;

        while !is_whitespace_or_end(self.next_char()) && !self.is_end() {
            self.advance();
        }

        let mut full = &self.code[self.start_pos..self.current_pos];

        match full {
            "true" => self.add_token(Token::Bool(true)),
            "false" => self.add_token(Token::Bool(false)),
            _ => {
                if KEYWORDS.contains(&full) {
                    self.add_token(Token::Keyword(full));
                } else if DATA_TYPES.contains(&full) {
                    self.add_token(Token::Datatype(full));
                } else {
                    if full.chars().collect::<Vec<char>>()[full.len() - 1] == ':' {
                        colon = true;
                    }

                    if colon == true {
                        full = &full[0..full.len() - 1];
                    }

                    self.add_token(Token::Identifier(full));

                    if colon {
                        self.add_token(Token::Colon);
                    }
                }
            }
        };
    }

    fn skip_comment(&mut self) {
        self.advance();

        while self.next_char() != '\n' {
            self.advance();
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        while !self.is_end() {
            self.start_pos = self.current_pos;
            self.make_tokens();
        }

        self.tokens.clone()
    }
}
