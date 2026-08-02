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
    let mut line_num = 0;
    let mut col_num = 0;
    let mut tokens = vec![];

    while let Some((_, c)) = chars.peek() {
        match c {
            // whitespace
            ' ' | '\t' => {
                tokens.push(Token::WhiteSpace);
                col_num += 1;
                chars.next();
                while let Some(&(_, c2)) = chars.peek()
                    && matches!(c2, ' ' | '\t')
                {
                    col_num += 1;
                    chars.next();
                }
            }
            '\n' => {
                tokens.push(Token::NewLine);
                col_num = 0;
                line_num += 1;
                chars.next();
            }
            '\r' => {
                tokens.push(Token::NewLine);
                col_num = 0;
                line_num += 1;
                chars.next();
                if let Some(&(_, c2)) = chars.peek()
                    && c2 == '\n'
                {
                    chars.next();
                }
            }

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
    // newlines
    #[case("\n", Token::NewLine)]
    #[case("\r\n", Token::NewLine)]
    #[case("\r", Token::NewLine)]
    // whitespace
    #[case(" ", Token::WhiteSpace)]
    #[case("\t", Token::WhiteSpace)]
    #[case(" \t ", Token::WhiteSpace)]
    fn single_token(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex(s).expect("failed to lex");
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[rstest]
    // only newlines
    #[case("\n", 1)]
    #[case("\r", 1)]
    #[case("\n\n", 2)]
    #[case("\n\r", 2)]
    #[case("\r\n", 1)]
    #[case("\r\r", 2)]
    #[case("\r\n\n", 2)]
    #[case("\n\r\n", 2)]
    #[case("\n\n\r", 3)]
    #[case("\n\r\r", 3)]
    #[case("\r\n\r", 2)]
    #[case("\r\r\n", 2)]
    #[case("\r\n\r\n", 2)]
    // with extras
    #[case(" \n ", 1)]
    #[case(" \r ", 1)]
    #[case(" \n\n ", 2)]
    #[case(" \n\r ", 2)]
    #[case(" \r\n ", 1)]
    #[case(" \r\r ", 2)]
    #[case("\r \n", 2)]
    #[case("\n \r", 2)]
    #[case(" \r \n ", 2)]
    fn count_new_lines(#[case] s: &str, #[case] expected: usize) {
        let tokens = lex(s).expect("failed to lex");
        let count = tokens
            .iter()
            .filter(|x| matches!(x, Token::NewLine))
            .count();
        assert_eq!(expected, count);
    }

    #[rstest]
    #[case(" ", 1)]
    #[case("  ", 1)]
    #[case("\t", 1)]
    #[case("\t\t", 1)]
    #[case(" \t", 1)]
    #[case("\t ", 1)]
    #[case(" \t ", 1)]
    #[case("\t \t", 1)]
    #[case(" \n ", 2)]
    #[case("\t\n\t", 2)]
    #[case(" ; ", 2)]
    #[case(" ;\t", 2)]
    #[case("\t; ", 2)]
    #[case("\t;\t", 2)]
    fn count_whitespace(#[case] s: &str, #[case] expected: usize) {
        let tokens = lex(s).expect("failed to lex");
        let count = tokens
            .iter()
            .filter(|x| matches!(x, Token::WhiteSpace))
            .count();
        assert_eq!(expected, count);
    }
}
