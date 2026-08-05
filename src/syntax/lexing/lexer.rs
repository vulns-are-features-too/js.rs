use std::{
    iter::{Enumerate, Peekable},
    marker::PhantomData,
    str::Chars,
};

use crate::syntax::{
    lexing::{
        ampersand::Ampersand,
        bang::Bang,
        div::Div,
        dot::Dot,
        equal::Equal,
        gt::Greater,
        keyword_or_identifier::KeyworkOrIdentifier,
        lt::Less,
        minus::Minus,
        newline::{CR, LF},
        number::Number,
        percent::Percent,
        pipe::Pipe,
        plus::Plus,
        question::Question,
        star::Star,
        tokens::Token,
        whitespace::WhiteSpace,
    },
    locations::Point,
};

#[derive(Debug)]
pub enum LexingError {
    InvalidChar {
        point: Point,
        c: char,
    },
    InvalidNumber {
        point: Point,
        num_str: String,
        parse_err: String,
    },
}

impl LexingError {
    const fn invalid_char(point: Point, c: char) -> Self {
        Self::InvalidChar { point, c }
    }

    const fn invalid_number(point: Point, num_str: String, parse_err: String) -> Self {
        Self::InvalidNumber {
            point,
            num_str,
            parse_err,
        }
    }
}

pub trait LexerState {}

pub struct Base;

impl LexerState for Base {}

pub struct Lexer<'i, State>
where
    State: LexerState,
{
    pub input: &'i str,
    pub chars: Peekable<Enumerate<Chars<'i>>>,
    pub line_num: usize,
    pub col_num: usize,
    _s: PhantomData<State>,
}

impl<'i, State> Lexer<'i, State>
where
    State: LexerState,
{
    pub const fn transition<NewState: LexerState>(self) -> Lexer<'i, NewState> {
        Lexer {
            _s: PhantomData::<NewState>,
            input: self.input,
            chars: self.chars,
            line_num: self.line_num,
            col_num: self.col_num,
        }
    }

    pub fn next_char(&mut self) {
        self.col_num += 1;
        self.chars.next();
    }

    pub fn eat(&mut self, ch: char) -> bool {
        match self.chars.peek() {
            Some(&(_, c)) if c == ch => {
                self.next_char();
                true
            }
            _ => false,
        }
    }
}

impl<'i, 'o> Lexer<'i, Base>
where
    'i: 'o,
{
    fn new(input: &'i str) -> Self {
        Self {
            input,
            chars: input.chars().enumerate().peekable(),
            line_num: 0,
            col_num: 0,
            _s: PhantomData,
        }
    }

    const fn curr_point(&self) -> Point {
        Point {
            line: self.line_num,
            column: self.col_num,
        }
    }

    fn lex_one(mut self) -> (Self, Token<'o>) {
        if let Some(&(i, c)) = self.chars.peek() {
            return match c {
                // whitespace
                ' ' | '\t' => self.transition::<WhiteSpace>().lex(i),
                '\n' => self.transition::<LF>().lex(i),
                '\r' => self.transition::<CR>().lex(i),

                // single char
                ';' => self.single_char(Token::SemiColon(i)),
                ':' => self.single_char(Token::Colon(i)),
                '(' => self.single_char(Token::LeftParen(i)),
                ')' => self.single_char(Token::RightParen(i)),
                '[' => self.single_char(Token::LeftBracket(i)),
                ']' => self.single_char(Token::RightBracket(i)),
                '{' => self.single_char(Token::LeftBrace(i)),
                '}' => self.single_char(Token::RightBrace(i)),
                '^' => self.single_char(Token::Caret(i)),
                '~' => self.single_char(Token::Tilde(i)),
                ',' => self.single_char(Token::Comma(i)),

                'a'..='z' | 'A'..='Z' | '_' => self.transition::<KeyworkOrIdentifier>().lex(i),

                '0'..='9' => self.transition::<Number>().lex(i, c),

                '=' => self.transition::<Equal>().lex(i),
                '!' => self.transition::<Bang>().lex(i),
                '>' => self.transition::<Greater>().lex(i),
                '<' => self.transition::<Less>().lex(i),
                '+' => self.transition::<Plus>().lex(i),
                '-' => self.transition::<Minus>().lex(i),
                '*' => self.transition::<Star>().lex(i),
                '/' => self.transition::<Div>().lex(i),
                '%' => self.transition::<Percent>().lex(i),
                '&' => self.transition::<Ampersand>().lex(i),
                '|' => self.transition::<Pipe>().lex(i),
                '.' => self.transition::<Dot>().lex(i),
                '?' => self.transition::<Question>().lex(i),
                _ => {
                    let token = Token::Invalid(&self.input[i..=i]);
                    (self, token)
                }
            };
        }
        (self, Token::Eof)
    }

    fn lex(mut self) -> Vec<Token<'o>> {
        let mut tokens: Vec<Token<'o>> = vec![];
        loop {
            let (new_self, token) = self.lex_one();
            self = new_self;

            match token {
                Token::Eof => {
                    tokens.push(Token::Eof);
                    return tokens;
                }
                _ => tokens.push(token),
            }
        }
    }
}

pub fn lex(input: &str) -> Vec<Token<'_>> {
    Lexer::new(input).lex()
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[rstest]
    #[case("int ABCD yz", vec![3, 4, 8, 9, 11])]
    #[case("12 345 67890", vec![2, 3, 6, 7, 12])]
    #[case("0x12 0X9A 0xDeF", vec![4, 5, 9, 10, 15])]
    #[case("0b0 0b101 0B1", vec![3, 4, 9, 10, 13])]
    #[case("0o1 0777 0O0", vec![3, 4, 8, 9, 12])]
    #[case("0 0xF 0b1 0o7 077 9", vec![1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 19])]
    #[case("0n 0xFn 0b1n 0o7n 077n 9n", vec![2, 3, 7, 8, 12, 13, 17, 18, 22, 23, 25])]
    #[case("public class Foo {}", vec![6, 7, 12, 13, 16, 17, 18, 19])]
    fn columns_numbers(#[case] s: &str, #[case] token_lengths: Vec<usize>) {
        let mut lexer = Lexer::new(s);
        for len in token_lengths {
            (lexer, _) = lexer.lex_one();
            assert_eq!(0, lexer.line_num);
            assert_eq!(len, lexer.col_num);
        }
    }
}
