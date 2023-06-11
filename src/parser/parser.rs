use crate::{
    ast::ast::Program,
    lexer::lexer::{Lexer, LexerIterItem},
    token::token::Token,
};
use std::cell::Cell;

pub struct Parser {
    lexer: Cell<LexerIterItem>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: Cell::new(lexer.into_iter()),
            current_token: None,
            peek_token: None,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_mut().next();
    }

    pub fn parse_program(&self) -> Option<Program> {
        todo!()
    }
}
#[cfg(test)]
mod parser_tester {
    use crate::{lexer::lexer::Lexer, ast::ast::{LetStatement, Statement}};

    use super::Parser;

    #[test]
    fn let_parser() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 8383883;
        ";

        let lexer = Lexer::new(String::from(input));
        let parser = Parser::new(lexer);

        let statements = parser.parse_program().unwrap().statements;

        assert_eq!(statements.len(), 3);

        let _expected_identifiers = vec!["x", "y", "foobar"];

        for statement in statements.into_iter() {
            assert_eq!(statement.token_literal(), "let");
            assert_eq!(statement.statement_type(), "LetStatement");
            let let_statement = unsafe { statement.downcast_ref_unchecked::<LetStatement>() };
            assert_eq!(let_statement.name.value, "x");
        }

    }
}
