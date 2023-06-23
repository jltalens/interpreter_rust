use crate::{
    ast::ast::{Expression, Identifier, Program, Statement},
    lexer::lexer::{Lexer, LexerIterItem},
    token::token::{Token, Tokens},
};
use std::{cell::Cell, collections::HashMap};

type PrefixParseFn = fn() -> Expression;

type InfixParseFn = fn(Expression) -> Expression;

pub struct Parser {
    lexer: Cell<LexerIterItem>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,

    prefix_parser_fns: HashMap<Tokens, PrefixParseFn>,
    infix_parser_fns: HashMap<Tokens, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer: Cell::new(lexer.into_iter()),
            current_token: None,
            peek_token: None,
            errors: vec![],
            infix_parser_fns: HashMap::new(),
            prefix_parser_fns: HashMap::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let program = Program::new();
        self.int_parse_program(program)
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_mut().next();
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
            Some(token) if token.token_type == Tokens::RETURN => self.parse_return(),
            Some(token) => self.parse_expression(),
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
        // TODO: We are ignoring the following expressions;
        Some(Statement::LetStatement(identifier.clone(), Expression::Identifier(identifier)))
    }

    fn parse_return(&mut self) -> Option<Statement> {
        self.next_token();
        while !(self.current_token.clone().unwrap().token_type != Tokens::SEMICOLON) {
            self.next_token();
        }
        // TODO: We are ignoring the following expressions;
        let expression = Expression::Identifier(Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal,
        });
        Some(Statement::ReturnStatement(Token { token_type: Tokens::RETURN, literal: self.current_token.clone().unwrap().literal }, expression))
    }

    fn parse_expression(&mut self) -> Option<Statement> {
        //TODO: Redo this
        let expression = self.parse_prefix(self.current_token.clone().unwrap().token_type);
        if self.peek_token.clone().unwrap().token_type == Tokens::SEMICOLON {
            self.next_token();
        }
        Some(Statement::ExpressionStatement(self.current_token.clone().unwrap(), expression))
    }


    fn expected_token(&mut self, token_type: Tokens) -> bool {
        match self.peek_token.clone() {
            Some(token) if token.token_type == token_type => {
                self.next_token();
                true
            }
            _ => {
                self.token_errored(token_type);
                false
            },
        }
    }

    fn token_errored(&mut self, token_type: Tokens) {
        self.errors.push(String::from(format!("expected next token to be {:?}, got {:?} instead", token_type, self.current_token.as_ref().unwrap())))
    }

    fn register_prefix(&mut self, token_type: Tokens, prefix_parser_fn: PrefixParseFn) {
        self.prefix_parser_fns.insert(token_type, prefix_parser_fn);
    }

    fn register_infix(&mut self, token_type: Tokens, infix_parser_fn: InfixParseFn) {
        self.infix_parser_fns.insert(token_type, infix_parser_fn);
    }
}
#[cfg(test)]
mod parser_tester {
    use crate::ast::ast::{Identifier, Expression, Statement};
    use crate::ast::ast::Statement::{LetStatement, ReturnStatement, ExpressionStatement};
    use crate::lexer::lexer::Lexer;
    use crate::token::token::{Tokens, Token};

    use super::Parser;

    #[test]
    fn let_parser() -> Result<(), Vec<String>>{
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 8383883;
        ";

        let lexer = Lexer::new(String::from(input));
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        check_parser_errors(parser)?;

        assert_eq!(program.len(), 3);

        let mut expected_identifiers = vec!["x", "y", "foobar"];

        for statement in program.into_iter() {
            match statement {
                LetStatement(identifier, _) => {
                    assert_eq!(identifier.value, expected_identifiers.remove(0));
                },
                _ => panic!("Unexpected statement")
            }
        }
        Ok(())
    }

    #[test]
    fn return_parser() -> Result<(), Vec<String>>{
        let input = "
            return 5;
            return 10;
            return 1232123;
        ";

        let lexer = Lexer::new(String::from(input));
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.len(), 3);


        check_parser_errors(parser)?;

        for statement in program.into_iter() {
            match statement {
                ReturnStatement(token, _) => assert_eq!(token.token_type, Tokens::RETURN),
                _ => panic!("Unexpected statement")
            }
        }
        Ok(())


    }

    #[test]
    fn identifier_expression() -> Result<(), Vec<String>>{
        let input = "foobar;";

        let lexer = Lexer::new(String::from(input));
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        
        check_parser_errors(parser)?;

        assert_eq!(program.len(), 1);

        assert_eq!(vec![Statement::ExpressionStatement(Expression::Identifier(Identifier { token: Token {token_type: Tokens::IDENT, literal: String::from("foobar")}, value: String::from("foobar") }))], program);

        Ok(())


    }


    fn check_parser_errors(parser: Parser) -> Result<(), Vec<String>> {
        if parser.errors.len() > 0 {
            return Err(parser.errors);
        }
        Ok(())
    }
}
