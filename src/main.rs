mod types {
    pub mod language {
        pub mod uncertain;
        pub mod types;
    }
}

mod globals;
mod lexer;


fn main() {
    let src = r#"
    
    print("Hello World")

    "#;

    for token in lexer::lex(&src) {
        println!("{:?}", token);
    }
}
