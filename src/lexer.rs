use std::error::Error;

// only single elements
pub enum TokenKind {
    Identifier(String),
    QuotedString(String),
    Integer(usize),
    Decimal(f64),
    // Math
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    // closures
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    // dots
    Dot,
    Colon,
    Semicolon,
}

struct Tokenizer {

}

impl Tokenizer {
    fn new() -> Tokenizer {
        return Tokenizer {}
    }

    fn next_token(&mut self) -> Result<Option<(TokenKind, usize, usize)>, Box<dyn Error>> {
        todo!("test")
    }
}

pub fn tokenize(src: &str) -> Result<Vec<(TokenKind, usize, usize)>, Box<dyn Error>> {
    let mut tokenizer = Tokenizer::new();
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token()? {
        tokens.push(token);
    }
    return Ok(tokens);
}