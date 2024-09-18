use rand::Rng;

extern crate rand;

pub enum Uncertain {
    False,
    Maybe,
    True
}

impl Uncertain {
    pub fn from_bool(b: bool) -> Self {
        if b {
            return Uncertain::True;
        } else {
            return Uncertain::False;
        }
    }

    pub fn to_bool(&self) -> bool {
        if self == Uncertain::True {
            return true;
        }
        if self == Uncertain::False {
            return false;
        }

        let maybe: u8 = rand::thread_rng().gen();

        return maybe == 1;
    }
    
}

pub enum Value {
    Null,
    Bool(bool),
    Uncertain(Uncertain),
    String(String),
    Function(Function),
} // TODO: Implement Dictionary and class instance

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    properties: Vec<Property<'a>>
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
    parent: Scope<'a>,
    is_public: bool,
    is_constant: bool,
    name: &'a str,
    value: Value
}

pub struct Function {
    is_static: bool
}
