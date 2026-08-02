use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::syntax::{locations::Point, tokens::Token};

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
    const fn invalid_char(point: Point, c: char) -> Self {
        Self::InvalidChar { point, c }
    }

    const fn invalid_number(point: Point, num_str: String, parse_err: String) -> Self {
        Self::InvalidNumber {
            point,
            num_str,
            parse_err,
        }
    }
}

struct Lexer<'i, 'o>
where
    'i: 'o,
{
    input: &'i str,
    chars: Peekable<Enumerate<Chars<'i>>>,
    line_num: usize,
    col_num: usize,
    tokens: Vec<Token<'o>>,
}

impl<'i, 'o> Lexer<'i, 'o>
where
    'i: 'o,
{
    fn new(input: &'i str) -> Self {
        Self {
            input,
            chars: input.chars().enumerate().peekable(),
            line_num: 0,
            col_num: 0,
            tokens: vec![],
        }
    }

    const fn curr_point(&self) -> Point {
        Point {
            line: self.line_num,
            column: self.col_num,
        }
    }

    fn lex_one(&mut self) -> Result<bool, LexingError> {
        if let Some(&(i, c)) = self.chars.peek() {
            match c {
                // whitespace
                ' ' | '\t' => {
                    self.lex_whitespace();
                }
                '\n' => {
                    self.lex_newline();
                }
                '\r' => {
                    self.lex_carriage_return();
                }

                // single char
                ';' => {
                    self.tokens.push(Token::SemiColon);
                    self.col_num += 1;
                    self.chars.next();
                }
                ':' => {
                    self.tokens.push(Token::Colon);
                    self.col_num += 1;
                    self.chars.next();
                }
                '(' => {
                    self.tokens.push(Token::LeftParen);
                    self.col_num += 1;
                    self.chars.next();
                }
                ')' => {
                    self.tokens.push(Token::RightParen);
                    self.col_num += 1;
                    self.chars.next();
                }
                '[' => {
                    self.tokens.push(Token::LeftBracket);
                    self.col_num += 1;
                    self.chars.next();
                }
                ']' => {
                    self.tokens.push(Token::RightBracket);
                    self.col_num += 1;
                    self.chars.next();
                }
                '{' => {
                    self.tokens.push(Token::LeftBrace);
                    self.col_num += 1;
                    self.chars.next();
                }
                '}' => {
                    self.tokens.push(Token::RightBrace);
                    self.col_num += 1;
                    self.chars.next();
                }

                // identifier or keyword
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.lex_identifier_or_keyword(i);
                }

                // numbers
                '0' => {
                    self.lex_number_starting_with_0()?;
                }

                '1'..='9' => {
                    self.lex_number_starting_with_1_to_9(i, c)?;
                }

                _ => {
                    return Err(LexingError::invalid_char(self.curr_point(), c));
                }
            }
            return Ok(true);
        }

        self.tokens.push(Token::Eof);
        Ok(false)
    }

    fn lex(mut self) -> Result<Vec<Token<'o>>, LexingError> {
        while self.lex_one()? {}
        Ok(self.tokens)
    }

    #[inline]
    fn lex_whitespace(&mut self) {
        self.tokens.push(Token::WhiteSpace);
        self.col_num += 1;
        self.chars.next();
        while let Some((_, c2)) = &self.chars.peek()
            && matches!(c2, ' ' | '\t')
        {
            self.col_num += 1;
            self.chars.next();
        }
    }

    #[inline]
    fn lex_newline(&mut self) {
        self.tokens.push(Token::NewLine);
        self.col_num = 0;
        self.line_num += 1;
        self.chars.next();
    }

    #[inline]
    fn lex_carriage_return(&mut self) {
        self.tokens.push(Token::NewLine);
        self.col_num = 0;
        self.line_num += 1;
        self.chars.next();
        if let Some(&(_, c2)) = self.chars.peek()
            && c2 == '\n'
        {
            self.chars.next();
        }
    }

    #[inline]
    fn lex_identifier_or_keyword(&mut self, i: usize) {
        let start = i;
        let mut end = start;
        while let Some(&(_, c2)) = self.chars.peek()
            && (c2.is_alphanumeric() || c2 == '_')
        {
            end += 1;
            self.col_num += 1;
            self.chars.next();
        }
        self.tokens.push(Token::Identifier(&self.input[start..end]));
    }

    #[inline]
    fn lex_number_starting_with_0(&mut self) -> Result<(), LexingError> {
        self.col_num += 1;
        self.chars.next();
        if let Some((_, c2)) = self.chars.peek() {
            return match c2 {
                'b' | 'B' => {
                    self.lex_number_with_alt_base(2, |c| matches!(c, '0'..='1'), Token::Binary)
                }
                'x' | 'X' => {
                    self.lex_number_with_alt_base(16, |c| c.is_ascii_hexdigit(), Token::Hexadecimal)
                }
                'o' | 'O' => {
                    self.lex_number_with_alt_base(8, |c| matches!(c, '0'..='7'), Token::Octal)
                }
                '0'..='9' => self.lex_number_with_leading_0_but_2nd_char_is_digit(),
                _ => {
                    self.tokens.push(Token::Decimal(0.0.into()));
                    Ok(())
                }
            };
        }

        self.tokens.push(Token::Decimal(0.0.into()));
        Ok(())
    }

    fn try_parse_int_token<T>(
        &self,
        num_str: String,
        radix: u32,
        token_fn: T,
    ) -> Result<Token<'o>, LexingError>
    where
        T: Fn(i64) -> Token<'o>,
    {
        i64::from_str_radix(&num_str, radix)
            .map(token_fn)
            .map_err(|e| LexingError::invalid_number(self.curr_point(), num_str, e.to_string()))
    }

    fn lex_number_with_alt_base<M, T>(
        &mut self,
        radix: u32,
        matcher: M,
        token_fn: T,
    ) -> Result<(), LexingError>
    where
        M: Fn(char) -> bool,
        T: Fn(i64) -> Token<'o>,
    {
        let mut num_str = String::new();
        self.col_num += 1;
        self.chars.next();
        while let Some(&(_, c3)) = self.chars.peek()
            && matcher(c3)
        {
            num_str.push(c3);
            self.col_num += 1;
            self.chars.next();
        }
        let token = self.try_parse_int_token(num_str, radix, token_fn)?;
        self.tokens.push(token);
        Ok(())
    }

    #[inline]
    fn lex_number_with_leading_0_but_2nd_char_is_digit(&mut self) -> Result<(), LexingError> {
        let mut num_str = String::new();
        let mut is_octal = true;
        while let Some(&(_, c3)) = self.chars.peek()
            && c3.is_ascii_digit()
        {
            num_str.push(c3);
            self.col_num += 1;
            self.chars.next();
            is_octal = is_octal && matches!(c3, '0'..='7');
        }
        if is_octal {
            let token = self.try_parse_int_token(num_str, 8, Token::Octal)?;
            self.tokens.push(token);
        } else {
            match num_str.parse::<f64>() {
                Ok(f) => {
                    self.tokens.push(Token::Decimal(f.into()));
                }
                Err(e) => {
                    return Err(LexingError::invalid_number(
                        self.curr_point(),
                        num_str,
                        e.to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn lex_number_starting_with_1_to_9(&mut self, i: usize, c: char) -> Result<(), LexingError> {
        let start = i;
        let mut end = i;
        let mut num_str = String::from(c);
        let mut exponent = String::new();
        let mut has_exp = false;
        let mut has_decimal_point = false;
        self.col_num += 1;
        self.chars.next();
        while let Some(&(_, c2)) = self.chars.peek() {
            match c2 {
                '_' => {
                    end += 1;
                    self.col_num += 1;
                    self.chars.next();
                }
                '0'..='9' => {
                    if has_exp {
                        exponent.push(c2);
                    } else {
                        num_str.push(c2);
                    }
                    end += 1;
                    self.col_num += 1;
                    self.chars.next();
                }
                'e' | 'E' => {
                    if has_exp {
                        break;
                    }
                    has_exp = true;
                    end += 1;
                    self.col_num += 1;
                    self.chars.next();
                    if let Some(&(_, c3)) = self.chars.peek()
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
                    self.col_num += 1;
                    self.chars.next();
                }
                '.' => {
                    if has_decimal_point || has_exp {
                        break;
                    }
                    has_decimal_point = true;
                    end += 1;
                    num_str.push('.');
                    self.col_num += 1;
                    self.chars.next();
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
                            self.tokens.push(Token::Exponential {
                                base: f.into(),
                                exp: i,
                            });
                        }
                        Err(e) => {
                            return Err(LexingError::invalid_number(
                                self.curr_point(),
                                self.input[start..end].to_string(),
                                e.to_string(),
                            ));
                        }
                    }
                } else {
                    self.tokens.push(Token::Decimal((f).into()));
                }
            }
            Err(e) => {
                return Err(LexingError::invalid_number(
                    self.curr_point(),
                    self.input[start..end].to_string(),
                    e.to_string(),
                ));
            }
        }
        Ok(())
    }
}

pub fn lex(input: &str) -> Result<Vec<Token<'_>>, LexingError> {
    Lexer::new(input).lex()
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
    #[case("077", Token::Octal(0o77))]
    #[case("0o0", Token::Octal(0o0))]
    #[case("0o1", Token::Octal(0o1))]
    #[case("08", Token::Decimal(8.0.into()))]
    #[case("09", Token::Decimal(9.0.into()))]
    #[case("0989", Token::Decimal(989.0.into()))]
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
    #[case("0o123 077 090 0O70", vec![Token::Octal(0o123), Token::WhiteSpace, Token::Octal(0o77), Token::WhiteSpace, Token::Decimal(90.0.into()), Token::WhiteSpace, Token::Octal(0o70), Token::Eof])]
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

    #[rstest]
    #[case("int ABCD yz", vec![3, 4, 8, 9, 11])]
    #[case("12 345 67890", vec![2, 3, 6, 7, 12])]
    #[case("0x12 0X9A 0xDeF", vec![4, 5, 9, 10, 15])]
    #[case("0b0 0b101 0B1", vec![3, 4, 9, 10, 13])]
    #[case("0o1 0777 0O0", vec![3, 4, 8, 9, 12])]
    #[case("0 0xF 0b1 0o7 077 9", vec![1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 19])]
    fn columns_numbers(#[case] s: &str, #[case] expected: Vec<usize>) {
        let mut lexer = Lexer::new(s);
        for col in expected {
            assert!(lexer.lex_one().unwrap());
            assert_eq!(0, lexer.line_num);
            assert_eq!(col, lexer.col_num);
        }
        assert!(!lexer.lex_one().unwrap());
    }
}
