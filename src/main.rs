mod lexer;

fn main() {
    let src = r#"
    
    print("Hello World")

    "#;

    let mut lexer = lexer::Lexer::new(src);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == lexer::Token::Eof {
            break;
        }
    }
}
