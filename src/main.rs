mod types {
    pub mod language {
        pub mod uncertain;
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
