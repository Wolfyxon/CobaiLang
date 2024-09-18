use crate::types::language::uncertain::Uncertain;

pub enum Value<'a> {
    Null,
    Bool(bool),
    Uncertain(Uncertain),
    Number(f32),
    String(String),
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
    pub parent: Scope<'a>,
    pub is_public: bool,
    pub is_constant: bool,
    pub name: &'a str,
    pub value: Value<'a>
}

pub struct Function<'a> {
    pub is_static: bool,
    pub arguments: Vec<Property<'a>>,
    pub body: Scope<'a>
}

pub struct FunctionContext<'a> {
    pub function: &'a Function<'a>,
    pub caller: &'a Function<'a>,
    pub scope: &'a Scope<'a>,
    pub arguments: Vec<Value<'a>>,
}

impl<'a> FunctionContext<'a> {
    pub fn get_argument(&self, name: &'a str) -> Option<&Property<'a>> {
        return self.scope.get_property(name);
    }
}