use std::{
    iter::{Enumerate, Peekable},
    marker::PhantomData,
    str::Chars,
};

use crate::{
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
        string::JsString,
        tokens::Token,
        whitespace::WhiteSpace,
    },
    locations::Point,
};

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
    #[must_use]
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

    #[must_use]
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
    #[must_use]
    fn new(input: &'i str) -> Self {
        Self {
            input,
            chars: input.chars().enumerate().peekable(),
            line_num: 0,
            col_num: 0,
            _s: PhantomData,
        }
    }

    #[must_use]
    const fn curr_point(&self) -> Point {
        Point {
            line: self.line_num,
            column: self.col_num,
        }
    }

    #[must_use]
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
                '"' => self.transition::<JsString<'"'>>().lex(i),
                '\'' => self.transition::<JsString<'\''>>().lex(i),
                '`' => self.transition::<JsString<'`'>>().lex(i),
                _ => {
                    self.next_char();
                    let token = Token::Invalid(&self.input[i..=i]);
                    (self, token)
                }
            };
        }
        (self, Token::Eof)
    }

    #[must_use]
    fn lex(self) -> Vec<Token<'o>> {
        self.stream().collect()
    }

    pub fn stream(self) -> impl Iterator<Item = Token<'i>> {
        LexIter { lexer: Some(self) }
    }
}

struct LexIter<'i> {
    lexer: Option<Lexer<'i, Base>>,
}

impl<'i> Iterator for LexIter<'i> {
    type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        let (lexer, token) = self.lexer.take()?.lex_one();
        if !matches!(token, Token::Eof) {
            self.lexer = Some(lexer);
        }
        Some(token)
    }
}

/// Lex 1 token at a time
///
/// ```
/// use syntax::lexing::{lexer::lex, tokens::Token};
///
/// let input = "var x = 5;";
///
/// for token in lex("var x = 5;") {
///     println!("{:?}", token);
/// }
///
/// let mut lexer = lex(input);
/// assert_eq!(Some(Token::Var(0)), lexer.next());
/// assert_eq!(Some(Token::WhiteSpace(" ")), lexer.next());
/// assert_eq!(Some(Token::Identifier("x")), lexer.next());
/// assert_eq!(Some(Token::WhiteSpace(" ")), lexer.next());
/// assert_eq!(Some(Token::Equal(6)), lexer.next());
/// assert_eq!(Some(Token::WhiteSpace(" ")), lexer.next());
/// assert_eq!(Some(Token::Decimal("5")), lexer.next());
/// assert_eq!(Some(Token::SemiColon(9)), lexer.next());
/// assert_eq!(Some(Token::Eof), lexer.next());
/// assert_eq!(None, lexer.next());
/// ```
pub fn lex(input: &str) -> impl Iterator<Item = Token<'_>> {
    Lexer::new(input).stream()
}

/// Lex everything at once
///
/// ```
/// use syntax::lexing::{lexer::lex_all, tokens::Token};
/// let tokens = lex_all("var x = 5;");
/// assert_eq!(
///     vec![
///         Token::Var(0),
///         Token::WhiteSpace(" "),
///         Token::Identifier("x"),
///         Token::WhiteSpace(" "),
///         Token::Equal(6),
///         Token::WhiteSpace(" "),
///         Token::Decimal("5"),
///         Token::SemiColon(9),
///         Token::Eof,
///     ],
///     tokens
/// );
/// ```
#[must_use]
pub fn lex_all(input: &str) -> Vec<Token<'_>> {
    lex(input).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;
    use std::fs::{self, read_dir};

    #[rstest]
    #[case("int ABCD yz", vec![3, 4, 8, 9, 11])]
    #[case("12 345 67890", vec![2, 3, 6, 7, 12])]
    #[case("0x12 0X9A 0xDeF", vec![4, 5, 9, 10, 15])]
    #[case("0b0 0b101 0B1", vec![3, 4, 9, 10, 13])]
    #[case("0o1 0777 0O0", vec![3, 4, 8, 9, 12])]
    #[case("0 0xF 0b1 0o7 077 9", vec![1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 19])]
    #[case("0n 0xFn 0b1n 0o7n 077n 9n", vec![2, 3, 7, 8, 12, 13, 17, 18, 22, 23, 25])]
    #[case("public class Foo {}", vec![6, 7, 12, 13, 16, 17, 18, 19])]
    fn columns_numbers(#[case] s: &str, #[case] token_end_indices: Vec<usize>) {
        let mut lexer = Lexer::new(s);
        for len in token_end_indices {
            (lexer, _) = lexer.lex_one();
            assert_eq!(0, lexer.line_num);
            assert_eq!(len, lexer.col_num);
        }
    }

    #[test]
    fn invalid_single() {
        let expected = vec![Token::Invalid("\0"), Token::Eof];
        let tokens = lex_all("\0");
        assert_eq!(expected, tokens);
    }

    #[test]
    fn invalid_multiple() {
        let expected = vec![
            Token::Invalid("\0"),
            Token::Invalid("\0"),
            Token::Invalid("\0"),
            Token::Eof,
        ];
        let tokens = lex_all("\0\0\0");
        assert_eq!(expected, tokens);
    }

    #[test]
    fn invalid_amidst_valid() {
        let expected = vec![
            Token::Decimal("1"),
            Token::Invalid("\0"),
            Token::Identifier("a"),
            Token::Invalid("\0"),
            Token::Identifier("Z"),
            Token::Eof,
        ];
        let tokens = lex_all("1\0a\0Z");
        assert_eq!(expected, tokens);
    }

    #[test]
    fn total_len() {
        for file in read_dir("tests/js_files/")
            .unwrap()
            .map(|x| x.unwrap().path())
        {
            let content = fs::read_to_string(&file).unwrap();
            let expected_len = content.len();
            let tokens = lex_all(&content);
            let total_len = tokens.iter().map(|t| t.len()).sum();
            assert_eq!(expected_len, total_len, "{:?}", file.file_name().unwrap());
        }
    }
}
