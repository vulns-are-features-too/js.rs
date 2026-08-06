use crate::lexing::{
    lexer::{Base, Lexer},
    tokens::Token,
};

impl<'i, 'o> Lexer<'i, Base>
where
    'i: 'o,
{
    pub fn single_char(mut self, token: Token<'o>) -> (Self, Token<'o>) {
        self.next_char();
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("(", Token::LeftParen(0))]
    #[case(")", Token::RightParen(0))]
    #[case("[", Token::LeftBracket(0))]
    #[case("]", Token::RightBracket(0))]
    #[case("{", Token::LeftBrace(0))]
    #[case("}", Token::RightBrace(0))]
    #[case(";", Token::SemiColon(0))]
    #[case(":", Token::Colon(0))]
    #[case("^", Token::Caret(0))]
    #[case("~", Token::Tilde(0))]
    #[case(",", Token::Comma(0))]
    fn single_token(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }
}
