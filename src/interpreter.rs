use std::iter::Peekable;

use crate::{globals, parser::{ASTNode, FunctionArgument}};
pub enum Value {
    Number(f32),
    String(String),
    Null
}

pub struct FunctionContext<'a> {
    function: &'a Function,
    pub args: Vec<Value>
}

impl<'a> FunctionContext<'a> {
    pub fn get_arg(&self, name: &'a str) -> &Value {
        let index_res = self.function.args.iter().position(|arg| arg.name == name.to_string());
        
        if !index_res.is_some() {
            panic!("Argument '{}' not found.", name);
        }

        return self.args.get(index_res.unwrap()).unwrap()
    }
}

pub struct Function {
    name: String,
    args: Vec<FunctionArgument>,
    internal: Option<fn(FunctionContext)>,
    body: Option<Vec<ASTNode>>
}

impl Function {

    pub fn new_internal(name: String, args: Vec<FunctionArgument>, function: fn(FunctionContext)) -> Self {
        Function {
            name: name,
            args: args,
            internal: Some(function),
            body: None
        }
    }

    pub fn new_defined(name: String, args: Vec<FunctionArgument>, body: Vec<ASTNode>) -> Self {
        Function {
            name: name,
            args: args,
            internal: None,
            body: Some(body)
        }
    }

    pub fn call(&self, args: Vec<Value>) {
        let ctx = FunctionContext {
            function: self,
            args: args
        };

        if self.is_defined() {
            panic!("Defined functions are not implemented yet");
        }

        if self.is_implemented() {
            self.internal.unwrap()(ctx)
        }
    }

    pub fn is_internal(&self) -> bool {
        self.internal.is_some()
    }

    pub fn is_defined(&self) -> bool {
        self.body.is_some()
    }

    pub fn is_implemented(&self) -> bool {
        self.is_internal() || self.is_defined()
    }
}

pub struct Interpreter<'a> {
    nodes: Peekable<std::slice::Iter<'a, ASTNode>>,
    functions: Vec<Function> // temporary solution!
}

impl<'a> Interpreter<'a> {
    pub fn new(root: &'a ASTNode) -> Self {
        match root {
            ASTNode::Main(nodes) => {
                Interpreter {
                    nodes: nodes.iter().peekable(),
                    functions: globals::get_functions()
                }
            }

            _ => panic!("'{:?}' cannot be the root node", root)
        }
    }

    pub fn get_function(&self, name: &String) -> Option<&Function> {
        for func in self.functions.iter() {
            if &func.name == name {
                return Some(func);
            }
        }

        None
    }

    pub fn node_to_value(node: &ASTNode) -> Option<Value> {
        match node {
            ASTNode::Number(n) => Some(Value::Number(*n)),
            ASTNode::String(s) => Some(Value::String(s.clone())),

            ASTNode::Identifier(name) => todo!("Variables are not implemented yet. Attempted getting: {}", name),
            _ => panic!("Not a supported value node")
        }
    }

    pub fn run(&mut self) {
        while let Some(&node) = self.nodes.peek() {
            match node {
                ASTNode::FunctionCall {name, args} => {
                    let func = self.get_function(name);

                    if func.is_none() {
                        panic!("Unknown function '{}'", name);
                    }
                }

                ASTNode::Eof => { break; }
                _ => todo!("Unimplemented node {:?}", node)
            }

            self.nodes.next();
        }
    }
}