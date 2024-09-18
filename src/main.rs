use runtime::{Property, Type, Value};

mod types {
    pub mod language {
        pub mod uncertain;
    }
}

pub mod runtime;
mod globals;
mod lexer;


fn main() {
    runtime::Function::new_internal(globals::print, vec![Property::arg("text", Type::String)]).call_anonymous(vec![Value::String("aaa")]);
    let src = r#"
    
    print("Hello World")

    "#;

    for token in lexer::lex(&src) {
        println!("{:?}", token);
    }
}
