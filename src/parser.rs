use std::{env::args, iter::Peekable};
use crate::lexer::Token;

pub enum ASTNode {
    Define {
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

    pub fn parse_program(&mut self) -> Result<ASTNode, &'a str> {
        while let Some(&tok) = self.tokens.peek() {
            match tok {
                Token::Identifier(s) => {
                    return self.parse_identifier();
                }

                _ => {
                    return Err("Not implemented");
                }
            }

            self.tokens.next();
        }

        Err("Parsing already complete")
    }

    pub fn parse_call_args(&mut self) -> Result<Vec<ASTNode>, &'a str> {
        let mut values: Vec<ASTNode> = Vec::new(); 

        while let Some(&tok) = self.tokens.peek() {
            match tok {

                Token::Comma => { 
                    self.tokens.next(); 
                    continue;
                }

                Token::String(s) => {
                    values.push(ASTNode::StringLiteral(s.to_string()));
                },

                Token::Number(n) => {
                    values.push(ASTNode::NumberLiteral(*n));
                }

                Token::Identifier(name) => {
                    let res = self.parse_identifier();
                    
                    if(res.is_err()) {
                        return Err(res.err().unwrap());
                    }

                    values.push(res.unwrap());
                }

                Token::RParen => {
                    return Ok(values);
                }

                _ => {
                    return Err("help");
                }
            }

            self.tokens.next();
        }

        Err("Unexpected end of file")
    }

    pub fn parse_identifier(&mut self) -> Result<ASTNode, &'a str> {
        let exp = "Expected assignment or function call";

        let mut name = String::new();

        while let Some(&tok) = self.tokens.peek() {
            
            match tok {
                Token::Identifier(s) => {
                    if(!name.is_empty()) {
                        return Err(exp);
                    }

                    name = s.to_string();
                }

                Token::LParen => {
                    self.tokens.next();
                    let arg_result = self.parse_call_args();
                    
                    if arg_result.is_err() {
                        return Err(arg_result.err().unwrap());
                    }

                    return Ok(ASTNode::FunctionCall { 
                        name: name, 
                        args: arg_result.unwrap() 
                    });
                }

                _ => {
                    return Err(exp);
                }
            }

            self.tokens.next();
        }

        Err(exp)
    }
}