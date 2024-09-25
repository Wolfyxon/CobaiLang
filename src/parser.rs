use std::iter::Peekable;
use crate::lexer::Token;

pub enum ASTNode {
    Define {
        object: String,
        name: String,
        type_name: String,
        value: Box<ASTNode>
    },
    
    Assign {
        object: String,
        name: String,
        value: Box<ASTNode>
    },

    FunctionCall {
        object: String,
        name: String,
        args: Vec<ASTNode>
    },

    Main(Vec<ASTNode>),
    NumberLiteral(f32),
    StringLiteral(String),
    Identifier(String)
}

pub struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens.iter().peekable()
        }
    }
}