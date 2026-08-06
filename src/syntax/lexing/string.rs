use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct JsString<const QUOTE: char>;

impl<const QUOTE: char> LexerState for JsString<QUOTE> {}

impl<'i, 'o, const QUOTE: char> Lexer<'i, JsString<QUOTE>>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();
        let mut end = start;
        let mut esc = false;
        let mut closed = false;
        while let Some(&(i, c)) = self.chars.peek() {
            end = i;
            if c == QUOTE && !esc {
                closed = true;
                self.next_char();
                break;
            }
            esc = !esc && c == '\\';
            self.next_char();
        }
        let token = if closed {
            match QUOTE {
                '`' => Token::Template(&self.input[start..=end]),
                _ => Token::String(&self.input[start..=end]),
            }
        } else {
            Token::Invalid(&self.input[start..=end])
        };
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("'a'", Token::String("'a'"))]
    #[case("`a`", Token::Template("`a`"))]
    #[case("\"a\"", Token::String("\"a\""))]
    #[case("'0'", Token::String("'0'"))]
    #[case("`0`", Token::Template("`0`"))]
    #[case("\"0\"", Token::String("\"0\""))]
    #[case("'abc'", Token::String("'abc'"))]
    #[case("`abc`", Token::Template("`abc`"))]
    #[case("\"abc\"", Token::String("\"abc\""))]
    #[case("' a '", Token::String("' a '"))]
    #[case("` a `", Token::Template("` a `"))]
    #[case("\" a \"", Token::String("\" a \""))]
    #[case("'123'", Token::String("'123'"))]
    #[case("`123`", Token::Template("`123`"))]
    #[case("\"123\"", Token::String("\"123\""))]
    #[case("'1\\'3'", Token::String("'1\\'3'"))]
    #[case("`1\\`3`", Token::Template("`1\\`3`"))]
    #[case("\"1\\\"3\"", Token::String("\"1\\\"3\""))]
    #[case("'1\\\\3'", Token::String("'1\\\\3'"))]
    #[case("`1\\\\3`", Token::Template("`1\\\\3`"))]
    #[case("\"1\\\\3\"", Token::String("\"1\\\\3\""))]
    #[case("'x", Token::Invalid("'x"))]
    #[case("`x", Token::Invalid("`x"))]
    #[case("\"x", Token::Invalid("\"x"))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }
}
