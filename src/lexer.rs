use std::{char, iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
    String(String),
    Identifier(String),
    If,                 // if
    Else,               // else
    ElseIf,             // elif              
    Not,                // not
    And,                // and
    Or,                 // or 
    Of,                 // of
    End,                // end
    Class,              // class
    Function,           // func
    Import,             // import
    LParen,             // (
    RParen,             // )
    LBrace,             // {
    RBrace,             // }
    LSqBrace,           // [
    RSqBrace,           // ]
    Colon,              // :
    Assign              // =
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(&ch) = self.chars.peek() {
            let mut tok: Token = Token::Unknown;
            let mut allow_next = true;

            match ch {
                ' ' | '\t' | '\n' => {
                    self.chars.next();
                    continue;
                }

                // Parens
                '(' => tok = Token::LParen,
                ')' => tok = Token::RParen,

                // Braces
                '{' => tok = Token::LBrace,
                '}' => tok = Token::RBrace,

                // Square braces
                '[' => tok = Token::LSqBrace,
                ']' => tok = Token::RSqBrace,

                // Other
                ':' => tok = Token::Colon,
                '=' => tok = Token::Assign,

                // Advanced

                '\'' | '"' => {
                    self.chars.next();
                    tok = self.read_string(&ch);
                }
                
                _ => {
                    if ch.is_alphabetic() {
                        tok = self.read_identifier();
                        allow_next = false;
                    }
                }
            }

            if allow_next {
                self.chars.next();
            }

            return tok;
        }
        return Token::Eof;
    }

    pub fn read_string(&mut self, boundary: &char) -> Token {
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.to_string() == boundary.to_string() {
                break;
            }

            string.push(ch);
            self.chars.next();
        }

        return Token::String(string);
    }

    pub fn read_identifier(&mut self) -> Token {
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if !ch.is_alphabetic() {
                break;
            }

            string.push(ch);
            self.chars.next();
        }

        match string.as_str() {
            "if"     => Token::If,
            "else"   => Token::Else,
            "elif"   => Token::ElseIf,
            "not"    => Token::Not,
            "and"    => Token::And,
            "or"     => Token::Or,
            "of"     => Token::Of,
            "end"    => Token::End,
            "class"  => Token::Class,
            "func"   => Token::Function,
            "import" => Token::Import,

            _ => Token::Identifier(string)
        }
    }
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());

        if token == Token::Eof {
            break;
        }
    }

    return tokens;
}