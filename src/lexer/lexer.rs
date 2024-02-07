use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
enum TokenType{
    // literal types
    Null,
    Number,
    Identifier,

    // keywords
    Let,

    // grouping operators
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator,

    EOF, // end of file
}

#[derive(Debug)]
pub struct Token {
    value: String,
    type_: TokenType,
}

impl Token {
    fn new(value: String, type_: TokenType) -> Self {
        Token { value, type_ }
    }
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("let", TokenType::Let),
        ("null", TokenType::Null),
    ]);

    let mut tokens:Vec<Token> = Vec::new();

    let mut src:VecDeque<char> = source_code.chars().collect();

    while src.len() > 0 {
        match src.front().copied() {
            Some('(') => {
                tokens.push(Token::new(String::from('('), TokenType::OpenParen));
                src.pop_front();
            }
            Some(')') => {
                tokens.push(Token::new(String::from(')'), TokenType::CloseParen));
                src.pop_front();
            }
            Some('+') => {
                tokens.push(Token::new(String::from('+'), TokenType::BinaryOperator));
                src.pop_front();
            }
            Some('-') => {
                tokens.push(Token::new(String::from('-'), TokenType::BinaryOperator));
                src.pop_front();
            }
            Some('*') => {
                tokens.push(Token::new(String::from('*'), TokenType::BinaryOperator));
                src.pop_front();
            }
            Some('/') => {
                tokens.push(Token::new(String::from('/'), TokenType::BinaryOperator));
                src.pop_front();
            }
            Some('%') => {
                tokens.push(Token::new(String::from('%'), TokenType::BinaryOperator));
                src.pop_front();
            }
            Some('=') => {
                tokens.push(Token::new(String::from('='), TokenType::Equals));
                src.pop_front();
            }
            // build number token
            Some(c) if c.is_numeric() => {
                let mut num = String::new();
                while let Some(digit) = src.front().copied().filter(|&c| c.is_numeric()) {
                    num.push(digit);
                    src.pop_front();
                }
                tokens.push(Token::new(num, TokenType::Number));
            },
            // build letters token
            Some(c) if c.is_alphabetic() => {
                let mut ident =  String::new();
                while let Some(letter) = src.front().copied().filter(|&c| c.is_alphabetic()){
                    ident.push(letter);
                    src.pop_front();
                }

                let token_type = match keywords.get(&*ident){
                    Some(&token_type) => token_type,
                    None => TokenType::Identifier
                };

                tokens.push(Token::new(ident, token_type))
            },
            Some(c) if c.is_whitespace() => {
                src.pop_front();
                // return unit type to match other arms
                ()
            },
            _ => {
                panic!("Unrecognized character found in source code: {:?}", src.front())
            }
        }
    }
    tokens.push(Token::new(String::from("EndOfFile"), TokenType::EOF));
    tokens
}

#[cfg(test)]
mod tests{
    use crate::lexer::lexer::{tokenize, TokenType};

    #[test]
    fn should_analyze_simple_code() {
        let source = "let x = 45 * (4 / 3)";
        let tokens = tokenize(source);
        let expected_len = 11; // code tokens + EOF
        assert_eq!(
            expected_len,
            tokens.len(),
            "{:?} should have len {} but was {}",
            tokens,
            expected_len,
            tokens.len()
        );
    }

    #[test]
    fn should_analyze_some_keywords(){
        let source = "null let";
        let tokens = tokenize(source);
        let expected_len = 3; // code tokens + EOF
        assert_eq!(
            expected_len,
            tokens.len(),
            "{:?} should have len {} but was {}",
            tokens,
            expected_len,
            tokens.len()
        );
    }

    #[test]
    fn should_analyze_int_numbers(){
        let source = "0 2 9 4567890";
        let tokens = tokenize(source);
        let expected_len = 5; // code tokens + EOF
        assert_eq!(
            expected_len,
            tokens.len(),
            "{:?} should have len {} but was {}",
            tokens,
            expected_len,
            tokens.len()
        );
    }

    #[test]
    fn should_analyze_variable_names(){
        let source = "x y z someVariable VARIABLE";
        let tokens = tokenize(source);
        let expected_len = 6; // code tokens + EOF
        assert_eq!(
            expected_len,
            tokens.len(),
            "{:?} should have len {} but was {}",
            tokens,
            expected_len,
            tokens.len()
        );
    }

    #[test]
    fn should_ignore_whitespaces(){
        let source = "\n\t  \r\n";
        let tokens = tokenize(source);
        let expected_len = 1; // EOF
        assert_eq!(
            expected_len,
            tokens.len(),
            "{:?} should only contain EOF",
            tokens,
        );
    }

    #[test]
    fn should_analyze_some_operators(){
        let source = "+ - * / %";
        let tokens = tokenize(source);
        let expected_len = 6; // code tokens + EOF
        assert_eq!(
            expected_len,
            tokens.len(),
            "{:?} should have len {} but was {}",
            tokens,
            expected_len,
            tokens.len()
        );
    }
}


