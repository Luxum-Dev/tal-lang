#[derive(Debug)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Whitespace,
    Bad,
    Eof,
}

#[derive(Debug)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TextSpan {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Token { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, "\u{0000}".to_string()),
            ));
        }

        let c = self.current_char();
        c.map(|c| {
            let start = self.current_pos;
            let kind = if Self::is_number_start(&c) {
                let number: i64 = self.consume_number();
                TokenKind::Number(number)
            } else if Self::is_whitespace(&c) {
                self.consume_whitespace()
            } else {
                self.consume_punctuation()
            };
            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        })
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad,
        }
    }

    fn consume_whitespace(&mut self) -> TokenKind {
        let _ = self.consume().unwrap();
        TokenKind::Whitespace
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn is_number_start(c: &char) -> bool {
        c.is_ascii_digit()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        match self.input.chars().nth(self.current_pos) {
            Some(c) => {
                self.current_pos += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}
