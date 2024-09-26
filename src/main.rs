use interpreter::Interpreter;
use parser::ASTNode;

mod types {
    pub mod language {
        pub mod uncertain;
        pub mod types;
        pub mod errors;
    }
}

mod globals;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    let err = types::language::errors::Error::new("Test error", "idk", 123);
    err.print();

    let src = r#"
    
    print("Hello World")

    "#;

    let tokens = lexer::lex(&src);
    let root = parser::parse(&tokens).unwrap();

    println!("= TOKENS =");
    for token in &tokens {
        println!("{:?}", token);
    }

    println!("= TREE =");
    match &root {
        ASTNode::Main(nodes) => {
            for node in nodes {
                println!("{:?}", node);
            }
        }
        _=> panic!("oh no {:?}", root)
    }

    println!("= EXECUTION =");

    let mut interpreter = Interpreter::new(&root);
    interpreter.run();

}
