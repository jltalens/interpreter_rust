use crate::token::token::Token;


pub trait Node{
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_type(&self) -> &str;
    fn statement_node(&self);
}

impl dyn Statement {
    pub unsafe fn downcast_ref_unchecked<T: Statement>(&self) -> &T {
        &*(self as *const dyn Statement as *const T)
    }
}

pub trait Expression: Node{
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}


pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement{
    fn statement_type(&self) -> &str{
        "LetStatement"
    }
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
