use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Minus;

impl LexerState for Minus {}

impl<'i, 'o> Lexer<'i, Minus>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::MinusEqual(start);
            return (self.transition(), token);
        }

        if self.eat('-') {
            let token = Token::Minus2(start);
            return (self.transition(), token);
        }

        let token = Token::Minus(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("-", Token::Minus(0))]
    #[case("--", Token::Minus2(0))]
    #[case("-=", Token::MinusEqual(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("a-=b--c-d");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::MinusEqual(1),
                Token::Identifier("b"),
                Token::Minus2(4),
                Token::Identifier("c"),
                Token::Minus(7),
                Token::Identifier("d"),
                Token::Eof,
            ],
            tokens
        );
    }
}
