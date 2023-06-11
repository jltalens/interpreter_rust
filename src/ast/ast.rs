use crate::token::token::Token;

pub type Program = Statements;

pub type Statements = Vec<Statement>;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub token: Token,
    pub value: String,
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Token, Expression),
}
