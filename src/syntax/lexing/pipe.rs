use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Pipe;

impl LexerState for Pipe {}

impl<'i, 'o> Lexer<'i, Pipe>
where
    'i: 'o,
{
    #[must_use]
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::PipeEqual(start);
            return (self.transition(), token);
        }

        if self.eat('|') {
            let token = Token::Pipe2(start);
            return (self.transition(), token);
        }

        let token = Token::Pipe(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("|", Token::Pipe(0))]
    #[case("||", Token::Pipe2(0))]
    #[case("|=", Token::PipeEqual(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("a|=b|c||d");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::PipeEqual(1),
                Token::Identifier("b"),
                Token::Pipe(4),
                Token::Identifier("c"),
                Token::Pipe2(6),
                Token::Identifier("d"),
                Token::Eof,
            ],
            tokens
        );
    }
}
