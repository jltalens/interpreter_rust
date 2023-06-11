use crate::{
    ast::ast::{Expression, Identifier, Program, Statement},
    lexer::lexer::{Lexer, LexerIterItem},
    token::token::{Token, Tokens},
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

    pub fn parse_program(&mut self) -> Program {
        let program = Program::new();
        self.int_parse_program(program)
    }

    fn int_parse_program(&mut self, mut program: Program) -> Program {
        match self.current_token.clone() {
            Some(token) if token.token_type != Tokens::EOF => match self.parse_statement() {
                Some(stmt) => {
                    program.push(stmt);
                    self.next_token();
                    self.int_parse_program(program)
                }
                _ => {
                    self.next_token();
                    self.int_parse_program(program)
                }
            },
            _ => program,
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.as_ref() {
            Some(token) if token.token_type == Tokens::LET => self.parse_let(),
            _ => None,
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        if !self.expected_token(Tokens::IDENT) {
            return None;
        }

        let identifier = Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal,
        };
        if !self.expected_token(Tokens::ASSIGN) {
            return None;
        }
        while !(self.current_token.clone().unwrap().token_type != Tokens::SEMICOLON) {
            self.next_token();
        }
        let expression = Expression {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal,
        };
        Some(Statement::LetStatement(identifier, expression))
    }

    fn expected_token(&mut self, token_type: Tokens) -> bool {
        match self.peek_token.clone() {
            Some(token) if token.token_type == token_type => {
                self.next_token();
                true
            }
            _ => false,
        }
    }
}
#[cfg(test)]
mod parser_tester {
    use crate::ast::ast::Statement::LetStatement;
    use crate::lexer::lexer::Lexer;

    use super::Parser;

    #[test]
    fn let_parser() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 8383883;
        ";

        let lexer = Lexer::new(String::from(input));
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.len(), 3);

        let mut expected_identifiers = vec!["x", "y", "foobar"];

        for statement in program.into_iter() {
            match statement {
                LetStatement(identifier, _) => {
                    assert_eq!(identifier.value, expected_identifiers.remove(0));
                }
            }
        }
    }
}
