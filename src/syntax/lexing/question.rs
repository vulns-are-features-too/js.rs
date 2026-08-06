use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Question;

impl LexerState for Question {}

impl<'i, 'o> Lexer<'i, Question>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('.') {
            let token = Token::QuestionDot(start);
            return (self.transition(), token);
        }

        if !self.eat('?') {
            let token = Token::Question(start);
            return (self.transition(), token);
        }

        if self.eat('=') {
            let token = Token::Question2Equal(start);
            return (self.transition(), token);
        }

        let token = Token::Question2(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("?", Token::Question(0))]
    #[case("??", Token::Question2(0))]
    #[case("??=", Token::Question2Equal(0))]
    #[case("?.", Token::QuestionDot(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("a??=b?.c??d?e");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::Question2Equal(1),
                Token::Identifier("b"),
                Token::QuestionDot(5),
                Token::Identifier("c"),
                Token::Question2(8),
                Token::Identifier("d"),
                Token::Question(11),
                Token::Identifier("e"),
                Token::Eof,
            ],
            tokens
        );
    }
}
