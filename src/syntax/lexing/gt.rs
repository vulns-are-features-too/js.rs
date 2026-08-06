use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Greater;

impl LexerState for Greater {}

impl<'i, 'o> Lexer<'i, Greater>
where
    'i: 'o,
{
    #[must_use]
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::GreaterEqual(start);
            return (self.transition(), token);
        }

        if !self.eat('>') {
            let token = Token::Greater(start);
            return (self.transition(), token);
        }

        if self.eat('=') {
            let token = Token::Greater2Equal(start);
            return (self.transition(), token);
        }

        if !self.eat('>') {
            let token = Token::Greater2(start);
            return (self.transition(), token);
        }

        if self.eat('=') {
            let token = Token::Greater3Equal(start);
            return (self.transition(), token);
        }

        let token = Token::Greater3(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case(">", Token::Greater(0))]
    #[case(">>", Token::Greater2(0))]
    #[case(">>>", Token::Greater3(0))]
    #[case(">=", Token::GreaterEqual(0))]
    #[case(">>=", Token::Greater2Equal(0))]
    #[case(">>>=", Token::Greater3Equal(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("1>2>>3>>>0>>>=3>>=2>=1");
        assert_eq!(
            vec![
                Token::Decimal("1"),
                Token::Greater(1),
                Token::Decimal("2"),
                Token::Greater2(3),
                Token::Decimal("3"),
                Token::Greater3(6),
                Token::Decimal("0"),
                Token::Greater3Equal(10),
                Token::Decimal("3"),
                Token::Greater2Equal(15),
                Token::Decimal("2"),
                Token::GreaterEqual(19),
                Token::Decimal("1"),
                Token::Eof,
            ],
            tokens
        );
    }
}
