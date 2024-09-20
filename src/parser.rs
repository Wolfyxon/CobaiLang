use std::iter::Peekable;
use crate::lexer::Token;

pub enum Instruction {
    Unknown,
    Set,
    Get,
    Call,
    Define,
    Math,
    Logic,
    If,
    Loop,
    While,
    For,
    End,
    Eof
}

pub struct InstructionNode<'a> {
    instruction: Instruction,
    parent: Option<&'a InstructionNode<'a>>,
    children: Vec<InstructionNode<'a>>
}

impl<'a> InstructionNode<'a> {
    pub fn eof() -> Self {
        InstructionNode {
            instruction: Instruction::Eof,
            parent: None,
            children: Vec::new()
        }
    }
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

    pub fn next_node(&mut self) -> Result<InstructionNode, &'a str> {
        while let Some(&tok) = self.tokens.peek() {
            let res: Result<InstructionNode, &'a str>;
            
            match tok {
                _ => res = Err("Unhandled token")
            }

            self.tokens.next();
            return res;
        }

        return Ok(InstructionNode::eof());
    }
}