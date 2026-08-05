use crate::syntax::lexing::{
    lexer::{Base, Lexer},
    tokens::Token,
};


impl<'i, 'o> Lexer<'i, Base>
where
    'i: 'o,
{
    pub fn single_char(mut self, token: Token<'o>) -> (Self, Token<'o>) {
        self.col_num += 1;
        self.chars.next();
        (self.transition(), token)
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex;
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
    fn single_token(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }
}
