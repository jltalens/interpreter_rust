use crate::token::token::Token;

pub type Program = Statements;

pub type Statements = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Token, Expression),
    ExpressionStatement(Token, Expression),
}
