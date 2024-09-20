mod types {
    pub mod language {
        pub mod uncertain;
        pub mod types;
        pub mod errors;
    }
}

mod globals;
mod lexer;


fn main() {
    let err = types::language::errors::Error::new("Test error", "idk", 123);
    err.print();

    let src = r#"
    
    print("Hello World")

    "#;

    for token in lexer::lex(&src) {
        println!("{:?}", token);
    }
}
