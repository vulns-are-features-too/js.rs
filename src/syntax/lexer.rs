use crate::syntax::{locations::Point,tokens::Token};

#[derive(Debug)]
pub enum LexingError {
    InvalidChar {
        point: Point,
        c: char,
    },
}

impl LexingError {
    fn invalid_char(point: Point, c: char) -> Self {
        Self::InvalidChar { point, c }
    }
}

pub fn lex(s: &str) -> Result<Vec<Token<'_>>, LexingError>
{
    let mut chars = s.chars().enumerate().peekable();
    let line_num = 0;
    let mut col_num = 0;
    let mut tokens = vec![];

    while let Some((_, c)) = chars.peek() {
        match c {
            // single char
            ';' => {
                tokens.push(Token::SemiColon);
                col_num += 1;
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                col_num += 1;
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                col_num += 1;
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                col_num += 1;
                chars.next();
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                col_num += 1;
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightBracket);
                col_num += 1;
                chars.next();
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                col_num += 1;
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                col_num += 1;
                chars.next();
            }
            _ => {
                return Err(LexingError::invalid_char(Point::new(line_num, col_num), *c));
            }
        };
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[rstest]
    // single chars
    #[case("(", Token::LeftParen)]
    #[case(")", Token::RightParen)]
    #[case("[", Token::LeftBracket)]
    #[case("]", Token::RightBracket)]
    #[case("{", Token::LeftBrace)]
    #[case("}", Token::RightBrace)]
    #[case(";", Token::SemiColon)]
    #[case(":", Token::Colon)]
    fn single_token(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex(s).expect("failed to lex");
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }
}
