use crate::token::token::{Token, Tokens};

pub struct Lexer {
    input: Vec<char>,
}

impl Lexer {
    fn new(value: String) -> Self {
        Self {
            input: value.chars().collect(),
        }
    }
}

impl IntoIterator for Lexer {
    type Item = Token;

    type IntoIter = LexerIterItem;

    fn into_iter(self) -> Self::IntoIter {
        LexerIterItem {
            lexer: self,
            index: 0,
        }
    }
}

pub struct LexerIterItem {
    lexer: Lexer,
    index: usize,
}

impl LexerIterItem {
    fn read_identifier(&mut self) -> Option<Token> {
        let in_range =
            |a: char| -> bool { ('a'..='z').contains(&a) || ('A'..='Z').contains(&a) || a == '_' };
        let initial_position = self.index.clone();
        while in_range(self.lexer.input[self.index]) {
            self.index += 1;
        }
        let literal: String = self.lexer.input[initial_position..self.index].iter().collect::<String>();
        let token_type = match literal.as_str() {
            "fn" => Tokens::FUNCTION,
            "let" => Tokens::LET,
            _ => Tokens::IDENT
        };
        // Ugly, change it
        self.index -=1;
        Some(Token {
            token_type,
            literal,
        })
    }

    fn read_number(&mut self) -> Option<Token> {
        let initial_position = self.index.clone();
        while ('0'..='9').contains(&self.lexer.input[self.index]) {
            self.index += 1;
        }
        // Ugly, change it. Maybe this should be done in the Iterator itself.
        self.index -=1;
        Some(Token { token_type: Tokens::INT, literal: self.lexer.input[initial_position..self.index].iter().collect::<String>()} )
    }
}

impl Iterator for LexerIterItem {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Can move this functionality to LexerIterItem?
        if self.index == self.lexer.input.len() {
            return None;
        }
        while self.lexer.input[self.index] == ' ' || self.lexer.input[self.index] == 0xA as char {
            self.index += 1
        }
        let output = match &self.lexer.input[self.index] {
            '=' => Some(Token {
                token_type: Tokens::ASSIGN,
                literal: String::from("+"),
            }),
            '+' => Some(Token {
                token_type: Tokens::PLUS,
                literal: String::from('+'),
            }),
            '(' => Some(Token {
                token_type: Tokens::LPAREN,
                literal: String::from('('),
            }),
            ')' => Some(Token {
                token_type: Tokens::RPAREN,
                literal: String::from(')'),
            }),
            '{' => Some(Token {
                token_type: Tokens::LBRACE,
                literal: String::from('{'),
            }),
            '}' => Some(Token {
                token_type: Tokens::RBRACE,
                literal: String::from('}'),
            }),
            ';' => Some(Token {
                token_type: Tokens::SEMICOLON,
                literal: String::from(';'),
            }),
            ',' => Some(Token {
                token_type: Tokens::COMMA,
                literal: String::from(','),
            }),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
            '0'..='9' => self.read_number(),
            t => Some(Token {
                token_type: Tokens::ILLEGAL,
                literal: String::from(*t),
            }),
        };
        self.index += 1;
        output
    }
}

#[cfg(test)]
mod lexer_tester {
    use super::*;
    use crate::token::token::Tokens;

    #[test]
    fn test_next_token() {
        let input = "=+(){};";
        let expected: Vec<Tokens> = vec![
            Tokens::ASSIGN,
            Tokens::PLUS,
            Tokens::LPAREN,
            Tokens::RPAREN,
            Tokens::LBRACE,
            Tokens::RBRACE,
            Tokens::SEMICOLON,
        ];

        let lexer = Lexer::new(String::from(input));

        let actual: Vec<Tokens> = lexer.into_iter().map(|lex| lex.token_type).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_basic_function() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x,y) { x + y; };
        let result = add(five, ten);";

        let expected = vec![
            Tokens::LET,
            Tokens::IDENT,
            Tokens::ASSIGN,
            Tokens::INT,
            Tokens::SEMICOLON,
            Tokens::LET,
            Tokens::IDENT,
            Tokens::ASSIGN,
            Tokens::INT,
            Tokens::SEMICOLON,
            Tokens::LET,
            Tokens::IDENT,
            Tokens::ASSIGN,
            Tokens::FUNCTION,
            Tokens::LPAREN,
            Tokens::IDENT,
            Tokens::COMMA,
            Tokens::IDENT,
            Tokens::RPAREN,
            Tokens::LBRACE,
            Tokens::IDENT,
            Tokens::PLUS,
            Tokens::IDENT,
            Tokens::SEMICOLON,
            Tokens::RBRACE,
            Tokens::SEMICOLON,
            Tokens::LET,
            Tokens::IDENT,
            Tokens::ASSIGN,
            Tokens::IDENT,
            Tokens::LPAREN,
            Tokens::IDENT,
            Tokens::COMMA,
            Tokens::IDENT,
            Tokens::RPAREN,
            Tokens::SEMICOLON
        ];

        let lexer = Lexer::new(String::from(input));

        let actual: Vec<Tokens> = lexer.into_iter().map(|lex| lex.token_type).collect();

        assert_eq!(expected, actual);
    }
}
