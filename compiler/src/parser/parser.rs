use crate::{
    lexer::tokens::{Token, DATA_TYPES, KEYWORDS},
    parser::node::{Node, Types},
};
#[macro_use]
use crate::throw_error;
use crate::util::*;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    ast: Node<'a>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            ast: Node::new(Types::Scope),
            current: 0,
        }
    }

    fn advance(&mut self) -> Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }

    fn is_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn next_token(&self) -> Option<Token> {
        if self.is_end() {
            return None;
        }

        Some(self.tokens[self.current].clone())
    }

    fn parse_block(&mut self) -> RaptorexResult<'a, Node> {
        let current_token = self.advance();
        let mut node = match current_token {
            Token::Identifier(s) => Node::new(Types::FunctionCall(s)),
            _ => return Err(throw_error!("Unexpected token:", current_token)),
        };
        todo!()
    }

    fn parse_scope(&mut self) -> RaptorexResult<'a, Node> {
        todo!()
    }

    fn parse(&mut self) -> RaptorexResult<'a, Node> {
        todo!()
    }

    pub fn parse_tokens(&mut self) -> RaptorexResult<'a, Node> {
        while !self.is_end() {
            self.parse();
        }

        Ok(self.ast.clone())
    }
}
