mod types {
    pub mod language {
        pub mod uncertain;
    }
}

mod lexer;
mod runtime;


fn main() {
    let src = r#"
    
    print("Hello World")

    "#;

    for token in lexer::lex(&src) {
        println!("{:?}", token);
    }
}
