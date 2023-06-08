pub struct Token {
    pub token_type: Tokens,
    pub literal: String
}

#[derive(Debug, PartialEq)]
pub enum Tokens {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
}
