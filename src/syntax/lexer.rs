use crate::syntax::{locations::Point,tokens::Token};

#[derive(Debug)]
pub enum LexingError {
    InvalidChar {
        point: Point,
        c: char,
    },
    InvalidNumber {
        point: Point,
        num_str: String,
        parse_err: String,
    },
}

impl LexingError {
    fn invalid_char(point: Point, c: char) -> Self {
        Self::InvalidChar { point, c }
    }

    fn invalid_number(point: Point, num_str: String, parse_err: String) -> Self {
        Self::InvalidNumber {
            point,
            num_str,
            parse_err,
        }
    }
}

pub fn lex(s: &str) -> Result<Vec<Token<'_>>, LexingError>
{
    let mut chars = s.chars().enumerate().peekable();
    let mut line_num = 0;
    let mut col_num = 0;
    let mut tokens = vec![];

    while let Some((i, c)) = chars.peek() {
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

            // identifier or keyword
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = *i;
                let mut end = start;
                while let Some(&(_, c2)) = chars.peek()
                    && (c2.is_alphanumeric() || c2 == '_')
                {
                    end += 1;
                    col_num += 1;
                    chars.next();
                }
                tokens.push(Token::Identifier(&s[start..end]));
            }

            // numbers
            '0' => {
                col_num += 1;
                chars.next();
                if let Some((_, c2)) = chars.peek() {
                    let mut num_str = String::new();
                    match c2 {
                        'b' | 'B' => {
                            col_num += 1;
                            chars.next();
                            while let Some(&(_, c3)) = chars.peek()
                                && matches!(c3, '0' | '1')
                            {
                                num_str.push(c3);
                                col_num += 1;
                                chars.next();
                            }
                            match i64::from_str_radix(&num_str, 2) {
                                Ok(i) => {
                                    tokens.push(Token::Binary(i));
                                }
                                Err(e) => {
                                    return Err(LexingError::InvalidNumber {
                                        point: Point {
                                            line: line_num,
                                            column: col_num,
                                        },
                                        num_str,
                                        parse_err: e.to_string(),
                                    });
                                }
                            }
                        }
                        'x' | 'X' => {
                            col_num += 1;
                            chars.next();
                            while let Some(&(_, c3)) = chars.peek()
                                && c3.is_ascii_hexdigit()
                            {
                                num_str.push(c3);
                                col_num += 1;
                                chars.next();
                            }
                            match i64::from_str_radix(&num_str, 16) {
                                Ok(i) => {
                                    tokens.push(Token::Hexadecimal(i));
                                }
                                Err(e) => {
                                    return Err(LexingError::InvalidNumber {
                                        point: Point {
                                            line: line_num,
                                            column: col_num,
                                        },
                                        num_str,
                                        parse_err: e.to_string(),
                                    });
                                }
                            }
                        }
                        'o' | 'O' => { // TODO: handle no o & 8/9
                            col_num += 1;
                            chars.next();
                            while let Some(&(_, c3)) = chars.peek()
                                && matches!(c3, '0'..='7')
                            {
                                num_str.push(c3);
                                col_num += 1;
                                chars.next();
                            }
                            match i64::from_str_radix(&num_str, 8) {
                                Ok(i) => {
                                    tokens.push(Token::Octal(i));
                                }
                                Err(e) => {
                                    return Err(LexingError::InvalidNumber {
                                        point: Point {
                                            line: line_num,
                                            column: col_num,
                                        },
                                        num_str,
                                        parse_err: e.to_string(),
                                    });
                                }
                            }
                        }
                        _ => tokens.push(Token::Decimal(0.0.into())),
                    }
                } else {
                    tokens.push(Token::Decimal(0.0.into()));
                };
            }

            '1'..='9' => {
                let start = *i;
                let mut end = *i;
                let mut num_str = String::from(*c);
                let mut exponent = String::new();
                let mut has_exp = false;
                let mut has_decimal_point = false;
                col_num += 1;
                chars.next();
                while let Some(&(_, c2)) = chars.peek() {
                    match c2 {
                        '_' => {
                            end += 1;
                            col_num += 1;
                            chars.next();
                        }
                        '0'..='9' => {
                            if has_exp {
                                exponent.push(c2);
                            } else {
                                num_str.push(c2);
                            };
                            end += 1;
                            col_num += 1;
                            chars.next();
                        }
                        'e' | 'E' => {
                            if has_exp {
                                break;
                            }
                            has_exp = true;
                            end += 1;
                            col_num += 1;
                            chars.next();
                            if let Some(&(_, c3)) = chars.peek()
                                && !matches!(c3, '-' | '0'..='9')
                            {
                                break;
                            }
                        }
                        '-' => {
                            if !has_exp || !exponent.is_empty() {
                                break;
                            }
                            exponent.push('-');
                            end += 1;
                            col_num += 1;
                            chars.next();
                        }
                        '.' => {
                            if has_decimal_point || has_exp {
                                break;
                            }
                            has_decimal_point = true;
                            end += 1;
                            num_str.push('.');
                            col_num += 1;
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }

                match num_str.parse::<f64>() {
                    Ok(f) => {
                        if has_exp {
                            match exponent.parse::<i64>() {
                                Ok(i) => {
                                    tokens.push(Token::Exponential {
                                        base: f.into(),
                                        exp: i,
                                    });
                                }
                                Err(e) => {
                                    return Err(LexingError::invalid_number(
                                        Point::new(line_num, col_num),
                                        s[start..end].to_string(),
                                        e.to_string(),
                                    ));
                                }
                            }
                        } else {
                            tokens.push(Token::Decimal((f).into()));
                        }
                    }
                    Err(e) => {
                        return Err(LexingError::invalid_number(
                            Point::new(line_num, col_num),
                            s[start..end].to_string(),
                            e.to_string(),
                        ));
                    }
                }
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
    // identifiers
    #[case("x", Token::Identifier("x"))]
    #[case("x2", Token::Identifier("x2"))]
    #[case("myvar", Token::Identifier("myvar"))]
    #[case("myvar2", Token::Identifier("myvar2"))]
    #[case("a1b2c3", Token::Identifier("a1b2c3"))]
    #[case("my_var", Token::Identifier("my_var"))]
    #[case("my_other_var", Token::Identifier("my_other_var"))]
    #[case("_x", Token::Identifier("_x"))]
    #[case("_1", Token::Identifier("_1"))]
    // numbers
    #[case("0", Token::Decimal(0.0.into()))]
    #[case("1", Token::Decimal(1.0.into()))]
    #[case("9", Token::Decimal(9.0.into()))]
    #[case("1_2", Token::Decimal(12.0.into()))]
    #[case("9_9", Token::Decimal(99.0.into()))]
    #[case("1_23_4", Token::Decimal(1234.0.into()))]
    #[case("1.2", Token::Decimal(1.2.into()))]
    #[case("9.9", Token::Decimal(9.9.into()))]
    #[case("1_2.3_4", Token::Decimal(12.34.into()))]
    #[case("2e3", Token::Exponential{base: 2.0.into(), exp: 3})]
    #[case("1_2e3_4", Token::Exponential{base: 12.0.into(), exp: 34})]
    #[case("1.2e3", Token::Exponential{base: 1.2.into(), exp: 3})]
    #[case("1.2e-3", Token::Exponential{base: 1.2.into(), exp: -3})]
    #[case("1_2.3_4e-5_6", Token::Exponential{base: 12.34.into(), exp: -56})]
    #[case("9_9.9_9e9_9", Token::Exponential{base: 99.99.into(), exp: 99})]
    #[case("9_9.9_9e-9_9", Token::Exponential{base: 99.99.into(), exp: -99})]
    #[case("0X0", Token::Hexadecimal(0))]
    #[case("0x0", Token::Hexadecimal(0))]
    #[case("0x1", Token::Hexadecimal(1))]
    #[case("0x123456789", Token::Hexadecimal(0x123456789))]
    #[case("0xABCDEF0", Token::Hexadecimal(0xABCDEF0))]
    #[case("0xabcdef0", Token::Hexadecimal(0xABCDEF0))]
    #[case("0b1010", Token::Binary(0b1010))]
    #[case("0B0101", Token::Binary(0b0101))]
    #[case("0b111", Token::Binary(0b111))]
    #[case("0b000", Token::Binary(0b0))]
    #[case("0o77", Token::Octal(0o77))]
    #[case("0O77", Token::Octal(0o77))]
    #[case("0o0", Token::Octal(0o0))]
    #[case("0o1", Token::Octal(0o1))]
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

    #[rstest]
    #[case("0 0 0", vec![Token::Decimal(0.0.into()), Token::WhiteSpace, Token::Decimal(0.0.into()), Token::WhiteSpace, Token::Decimal(0.0.into()), Token::Eof])]
    #[case("1 2 3", vec![Token::Decimal(1.0.into()), Token::WhiteSpace, Token::Decimal(2.0.into()), Token::WhiteSpace, Token::Decimal(3.0.into()), Token::Eof])]
    #[case("19 28 37", vec![Token::Decimal(19.0.into()), Token::WhiteSpace, Token::Decimal(28.0.into()), Token::WhiteSpace, Token::Decimal(37.0.into()), Token::Eof])]
    #[case("0x0 0x0 0x0", vec![Token::Hexadecimal(0), Token::WhiteSpace, Token::Hexadecimal(0), Token::WhiteSpace, Token::Hexadecimal(0), Token::Eof])]
    #[case("0x1 0X2 0x3", vec![Token::Hexadecimal(1), Token::WhiteSpace, Token::Hexadecimal(2), Token::WhiteSpace, Token::Hexadecimal(3), Token::Eof])]
    #[case("0xAb 0XcD 0xEF", vec![Token::Hexadecimal(0xAB), Token::WhiteSpace, Token::Hexadecimal(0xCD), Token::WhiteSpace, Token::Hexadecimal(0xEF), Token::Eof])]
    #[case("0b111 0B1010 0b000", vec![Token::Binary(0b111), Token::WhiteSpace, Token::Binary(0b1010), Token::WhiteSpace, Token::Binary(0b0), Token::Eof])]
    #[case("0o123 0o77 0O70", vec![Token::Octal(0o123), Token::WhiteSpace, Token::Octal(0o77), Token::WhiteSpace, Token::Octal(0o70), Token::Eof])]
    fn numbers(#[case] s: &str, #[case] expected: Vec<Token>) {
        let tokens = lex(s).expect("failed to lex");
        assert_eq!(expected, tokens);
    }

    #[rstest]
    #[case("a b c", vec![Token::Identifier("a"), Token::WhiteSpace, Token::Identifier("b"), Token::WhiteSpace, Token::Identifier("c"), Token::Eof])]
    #[case("a1 b9 c0", vec![Token::Identifier("a1"), Token::WhiteSpace, Token::Identifier("b9"), Token::WhiteSpace, Token::Identifier("c0"), Token::Eof])]
    #[case("_X _y _Z", vec![Token::Identifier("_X"), Token::WhiteSpace, Token::Identifier("_y"), Token::WhiteSpace, Token::Identifier("_Z"), Token::Eof])]
    fn identifiers(#[case] s: &str, #[case] expected: Vec<Token>) {
        let tokens = lex(s).expect("failed to lex");
        assert_eq!(expected, tokens);
    }
}
