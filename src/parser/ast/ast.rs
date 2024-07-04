use crate::lexer::token::token::Token;

pub trait Node {
    fn get_lexeme(&self) -> String;
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn get_lexeme(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.get_lexeme(),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}

impl Node for Statement {
    fn get_lexeme(&self) -> String {
        match self {
            Statement::Let(statement) => statement.get_lexeme(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}

impl Node for Program {
    fn get_lexeme(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.get_lexeme())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug)]
pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Expression) -> LetStatement {
        LetStatement { token, name, value }
    }
}

impl Node for LetStatement {
    fn get_lexeme(&self) -> String {
        format!(
            "let {} = {}",
            self.name.get_lexeme(),
            self.value.get_lexeme()
        )
    }
}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }
}

impl Node for Identifier {
    fn get_lexeme(&self) -> String {
        return self.token.lexeme.clone();
    }
}
