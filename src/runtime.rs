pub enum Uncertain {
    False,
    Unlikely,
    Maybe,
    Likely,
    True
}

pub enum ScopeProperty<'a> {
    Variable(Variable<'a>),
    Function(Function<'a>)
}

pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    properties: Vec<ScopeProperty<'a>>
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

    pub fn get_local_property(&self, name: &'a str) -> Option<&ScopeProperty<'a>> {
        for prop in self.properties.iter() {
            match prop {
                ScopeProperty::Function(func) => {
                    if func.property.name == name {
                        return Some(prop);
                    }
                }
                ScopeProperty::Variable(var) => {
                    if var.property.name == name {
                        return Some(prop);
                    }
                }
            }
        }

        return None;
    }

    pub fn get_property(&self, name: &'a str) -> Option<&ScopeProperty<'a>> {
        let local = self.get_local_property(name);
        
        if local.is_some() {
            return local;
        }

        for anc in self.get_ancestors().iter() {
            let prop = anc.get_property(name);
            
            if(prop.is_some()) {
                return prop;
            }
        }

        return None;
    }
}

pub struct Property<'a> {
    parent: Scope<'a>,
    public: bool,
    name: &'a str,
    constant: bool
}

pub struct Variable<'a> {
    property: &'a Property<'a>
}

pub struct Function<'a> {
    property: &'a Property<'a>
}
