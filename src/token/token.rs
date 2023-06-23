#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub token_type: Tokens,
    pub literal: String
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
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
    LET,
    BANG,
    MINUS,
    SLASH,
    ASTERISK,
    LT,
    GT,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOTEQ
}
