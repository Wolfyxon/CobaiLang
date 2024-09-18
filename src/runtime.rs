use std::ops::Index;

use crate::types::language::uncertain::Uncertain;

pub enum Type {
    Null,
    Bool,
    Uncertain,
    Number,
    String,
    Function
}

pub enum Value<'a> {
    Null,
    Bool(bool),
    Uncertain(Uncertain),
    Number(f32),
    String(&'a str),
    Function(Function<'a>),
} // TODO: Implement Dictionary and class instance

pub struct Scope<'a> {
    pub parent: Option<&'a Scope<'a>>,
    pub properties: Vec<Property<'a>>
}

impl<'a> Scope<'a> {
    pub fn new(parent: &'a Scope<'a>) -> Self {
        Scope {
            parent: Some(parent),
            properties: Vec::new(),
        }
    }

    pub fn get_ancestors(&self) -> Vec<&'a Scope<'a>> {
        let mut ancestors: Vec<&'a Scope<'a>> = Vec::new();
        let mut current = self.parent;

        while current.is_some() {
            current = current.unwrap().parent;
            
            if current.is_some() {
                ancestors.push(current.unwrap());
            }
        }

        return ancestors;
    }

    pub fn get_local_property(&self, name: &'a str) -> Option<&Property<'a>> {
        for prop in self.properties.iter() {
            if prop.name == name {
                return Some(prop);
            }
        }

        return None;
    }

    pub fn get_property(&self, name: &'a str) -> Option<&Property<'a>> {
        let local = self.get_local_property(name);
        
        if local.is_some() {
            return local;
        }

        for anc in self.get_ancestors().iter() {
            let prop = anc.get_property(name);
            
            if prop.is_some() {
                return prop;
            }
        }

        return None;
    }
}

pub struct Property<'a> {
    pub parent: Option<Scope<'a>>,
    pub is_public: bool,
    pub is_constant: bool,
    pub name: &'a str,
    pub value_type: Type,
    pub value: Value<'a>
}

impl<'a> Property<'a> {
    pub fn arg(name: &'a str, value_type: Type) -> Self {
        Property {
            parent: None,
            is_public: false,
            is_constant: false,
            value: Value::Null,
            value_type: value_type,
            name: name
        }
    }
}

pub struct Function<'a> {
    pub is_static: bool,
    pub arguments: Vec<Property<'a>>,
    pub body: Option<Scope<'a>>,
    pub internal_callback: Option<fn(ctx: FunctionContext) -> Value<'a>>
}

impl<'a> Function<'a> {
    pub fn new_internal(function: fn(ctx: FunctionContext) -> Value<'a>, arguments: Vec<Property<'a>>) -> Self {
        Function {
            is_static: true,
            body: None,
            arguments: arguments,
            internal_callback: Some(function)
        }
    }

    pub fn call_anonymous(&self, args: Vec<Value>) -> Value<'a> {
        let ctx = FunctionContext {
            function: &self,
            arguments: args,
            scope: None,
            caller: None
        };

        if self.internal_callback.is_some() {
            return (self.internal_callback.unwrap())(ctx);
        }

        todo!();
    }
}

pub struct FunctionContext<'a> {
    pub function: &'a Function<'a>,
    pub caller: Option<&'a Function<'a>>,
    pub scope: Option<&'a Scope<'a>>,
    pub arguments: Vec<Value<'a>>,
}

impl<'a> FunctionContext<'a> {
    pub fn get_argument(&self, name: &'a str) -> Option<&Value<'a>> {
        let idx = self.function.arguments.iter().position(|p| p.name == name ).unwrap();
        return Some(&self.arguments[idx]);
    }
}