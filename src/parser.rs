use std::{env::args, iter::Peekable};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
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
    Number(f32),
    String(String),
    Identifier(String),
    Eof
}

pub struct FunctionArgument {
    pub name: String,
    pub type_name: String,
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

    pub fn parse_program(&mut self) -> Result<ASTNode, String> {
        while let Some(&tok) = self.tokens.peek() {
            match tok {
                Token::Identifier(s) => {
                    return self.parse_identifier();
                }

                Token::Eof => {
                    return Ok(ASTNode::Eof);
                }

                _ => {
                    return Err(format!("Unhandled token '{:?}'", tok));
                }
            }

            //self.tokens.next();
        }

        Err("Parsing already complete".to_string())
    }

    pub fn parse_call_args(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut values: Vec<ASTNode> = Vec::new(); 

        while let Some(&tok) = self.tokens.peek() {
            match tok {

                Token::Comma => { 
                    self.tokens.next(); 
                    continue;
                }

                Token::String(s) => {
                    values.push(ASTNode::String(s.to_string()));
                },

                Token::Number(n) => {
                    values.push(ASTNode::Number(*n));
                }

                Token::Identifier(name) => {
                    let res = self.parse_identifier();
                    
                    if(res.is_err()) {
                        return Err(res.err().unwrap());
                    }

                    values.push(res.unwrap());
                }

                Token::RParen => {
                    self.tokens.next();
                    return Ok(values);
                }

                _ => {
                    return Err("help".to_string());
                }
            }

            self.tokens.next();
        }

        Err("Unexpected end of file".to_string())
    }

    pub fn parse_identifier(&mut self) -> Result<ASTNode, String> {
        let exp = "Expected assignment or function call".to_string();

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

pub fn parse(tokens: &Vec<Token>) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens);
    let mut nodes: Vec<ASTNode> = Vec::new();

    loop {
        let res = parser.parse_program();

        if res.is_err() {
            return Err(res.err().unwrap());
        }

        let node = res.unwrap();

        match node {
            ASTNode::Eof => break,
            _ => nodes.push(node)
        };
    }

    Ok(ASTNode::Main((nodes)))
}