use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Number;
struct NumberLeading0;
struct Decimal;
struct BinInt;
struct HexInt;
struct OctInt;
struct Exponential;

impl LexerState for Number {}
impl LexerState for NumberLeading0 {}
impl LexerState for Decimal {}
impl LexerState for BinInt {}
impl LexerState for HexInt {}
impl LexerState for OctInt {}
impl LexerState for Exponential {}

impl<'i, 'o> Lexer<'i, Number>
where
    'i: 'o,
{
    pub fn lex(self, start: usize, c: char) -> (Lexer<'i, Base>, Token<'o>) {
        match c {
            '0' => self.transition::<NumberLeading0>().lex(start),
            _ => self.transition::<Decimal>().lex(start, 0),
        }
    }
}

impl<'i, 'o> Lexer<'i, NumberLeading0>
where
    'i: 'o,
{
    fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();
        if let Some(&(_, c2)) = self.chars.peek() {
            return match c2 {
                'b' | 'B' => self.transition::<BinInt>().lex(start),
                'x' | 'X' => self.transition::<HexInt>().lex(start),
                'o' | 'O' => self.transition::<OctInt>().lex(start, true),
                '0'..='7' => self.transition::<OctInt>().lex(start, false),
                '8'..='9' => self.transition::<Decimal>().lex(start, 1),
                'e' | 'E' => self.transition::<Exponential>().lex(start),
                'n' => {
                    self.next_char();
                    let token = Token::BigInt(&self.input[start..=(start + 1)]);
                    (self.transition(), token)
                }
                _ => {
                    let token = Token::Decimal(&self.input[start..=start]);
                    (self.transition(), token)
                }
            };
        }
        let token = Token::Decimal(&self.input[start..=start]);
        (self.transition(), token)
    }
}

impl<'i, 'o> Lexer<'i, BinInt>
where
    'i: 'o,
{
    fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        let mut end = start + 1;
        self.next_char();
        let mut e = false;
        while let Some(&(_, c)) = self.chars.peek()
            && matches!(c, '0' | '1' | 'e' | '_')
            && !(e && c == 'e')
        {
            end += 1;
            self.next_char();
            e = e || c == 'e';
        }
        let token = match self.chars.peek() {
            Some((_, 'n')) => {
                end += 1;
                self.next_char();
                if e {
                    Token::BigExpBin(&self.input[start..=end])
                } else {
                    Token::BigBin(&self.input[start..=end])
                }
            }
            _ => {
                if e {
                    Token::BigBin(&self.input[start..=end])
                } else {
                    Token::Binary(&self.input[start..=end])
                }
            }
        };
        (self.transition(), token)
    }
}

impl<'i, 'o> Lexer<'i, OctInt>
where
    'i: 'o,
{
    fn lex(mut self, start: usize, has_o: bool) -> (Lexer<'i, Base>, Token<'o>) {
        let mut end = start;
        if has_o {
            end += 1;
            self.next_char();
        }
        let mut e = false;
        while let Some(&(_, c)) = self.chars.peek()
            && matches!(c, '0'..='7' | 'e' | '_')
            && !(e && c == 'e')
        {
            end += 1;
            self.next_char();
            e = e || c == 'e';
        }
        if let Some((_, '8'..='9')) = self.chars.peek() {
            return self.transition::<Decimal>().lex(start, end - start + 1);
        }
        let token = match self.chars.peek() {
            Some((_, 'n')) => {
                end += 1;
                self.next_char();
                if e {
                    Token::BigExpOct(&self.input[start..=end])
                } else {
                    Token::BigOct(&self.input[start..=end])
                }
            }
            _ => {
                if e {
                    Token::BigOct(&self.input[start..=end])
                } else {
                    Token::Octal(&self.input[start..=end])
                }
            }
        };
        (self.transition(), token)
    }
}

impl<'i, 'o> Lexer<'i, HexInt>
where
    'i: 'o,
{
    fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        let mut end = start + 1;
        self.next_char();
        while let Some(&(_, c)) = self.chars.peek()
            && (c.is_ascii_hexdigit() || c == '_')
        {
            end += 1;
            self.next_char();
        }
        let token = match self.chars.peek() {
            Some((new_end, 'n')) => {
                end = *new_end;
                self.next_char();
                Token::BigHex(&self.input[start..=end])
            }
            _ => Token::Hexadecimal(&self.input[start..=end]),
        };
        (self.transition(), token)
    }
}

impl<'i, 'o> Lexer<'i, Decimal>
where
    'i: 'o,
{
    fn lex(mut self, start: usize, leading_zeros: usize) -> (Lexer<'i, Base>, Token<'o>) {
        let mut dot = false;
        let mut end = start + leading_zeros;
        self.next_char();
        while let Some(&(_, c)) = self.chars.peek() {
            match c {
                '0'..='9' | '_' => {
                    self.next_char();
                    end += 1;
                }
                '.' => {
                    if dot {
                        break;
                    }
                    dot = true;
                    self.next_char();
                    end += 1;
                }
                'e' => {
                    return self.transition::<Exponential>().lex(start);
                }
                _ => {
                    break;
                }
            }
        }
        let token = match self.chars.peek() {
            Some((new_end, 'n')) => {
                end = *new_end;
                self.next_char();
                Token::BigInt(&self.input[start..=end])
            }
            _ => Token::Decimal(&self.input[start..=end]),
        };
        (self.transition(), token)
    }
}

impl<'i, 'o> Lexer<'i, Exponential>
where
    'i: 'o,
{
    fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        let mut end = start;
        self.next_char();
        if let Some(&(e, '0'..='9' | '_' | '-')) = self.chars.peek() {
            self.next_char();
            end = e;
        }
        while let Some(&(e, '0'..='9' | '_')) = self.chars.peek() {
            self.next_char();
            end = e;
        }
        let token = Token::Exponential(&self.input[start..=end]);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("0", Token::Decimal("0"))]
    #[case("1", Token::Decimal("1"))]
    #[case("9", Token::Decimal("9"))]
    #[case("1_2", Token::Decimal("1_2"))]
    #[case("9_9", Token::Decimal("9_9"))]
    #[case("1_23_4", Token::Decimal("1_23_4"))]
    #[case("1.2", Token::Decimal("1.2"))]
    #[case("9.9", Token::Decimal("9.9"))]
    #[case("1_2.3_4", Token::Decimal("1_2.3_4"))]
    #[case("2e3", Token::Exponential("2e3"))]
    #[case("1_2e3_4", Token::Exponential("1_2e3_4"))]
    #[case("1.2e3", Token::Exponential("1.2e3"))]
    #[case("1.2e-3", Token::Exponential("1.2e-3"))]
    #[case("1_2.3_4e-5_6", Token::Exponential("1_2.3_4e-5_6"))]
    #[case("9_9.9_9e9_9", Token::Exponential("9_9.9_9e9_9"))]
    #[case("9_9.9_9e-9_9", Token::Exponential("9_9.9_9e-9_9"))]
    #[case("0X0", Token::Hexadecimal("0X0"))]
    #[case("0x0", Token::Hexadecimal("0x0"))]
    #[case("0x1", Token::Hexadecimal("0x1"))]
    #[case("0x123456789", Token::Hexadecimal("0x123456789"))]
    #[case("0xABCDEF0", Token::Hexadecimal("0xABCDEF0"))]
    #[case("0xabcdef0", Token::Hexadecimal("0xabcdef0"))]
    #[case("0b1010", Token::Binary("0b1010"))]
    #[case("0B0101", Token::Binary("0B0101"))]
    #[case("0b111", Token::Binary("0b111"))]
    #[case("0b000", Token::Binary("0b000"))]
    #[case("0o77", Token::Octal("0o77"))]
    #[case("0O77", Token::Octal("0O77"))]
    #[case("077", Token::Octal("077"))]
    #[case("007", Token::Octal("007"))]
    #[case("0077", Token::Octal("0077"))]
    #[case("0o0", Token::Octal("0o0"))]
    #[case("0o1", Token::Octal("0o1"))]
    #[case("08", Token::Decimal("08"))]
    #[case("09", Token::Decimal("09"))]
    #[case("0989", Token::Decimal("0989"))]
    #[case("009", Token::Decimal("009"))]
    #[case("0789", Token::Decimal("0789"))]
    #[case("000789", Token::Decimal("000789"))]
    #[case("1n", Token::BigInt("1n"))]
    #[case("99n", Token::BigInt("99n"))]
    #[case("0b1n", Token::BigBin("0b1n"))]
    #[case("0B1n", Token::BigBin("0B1n"))]
    #[case("0b101n", Token::BigBin("0b101n"))]
    #[case("0xfn", Token::BigHex("0xfn"))]
    #[case("0XFn", Token::BigHex("0XFn"))]
    #[case("0xFFn", Token::BigHex("0xFFn"))]
    #[case("0o7n", Token::BigOct("0o7n"))]
    #[case("0O7n", Token::BigOct("0O7n"))]
    #[case("0o77n", Token::BigOct("0o77n"))]
    #[case("07n", Token::BigOct("07n"))]
    #[case("077n", Token::BigOct("077n"))]
    #[case("08n", Token::BigInt("08n"))]
    #[case("088n", Token::BigInt("088n"))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[rstest]
    #[case("0 0 0", vec![Token::Decimal("0"), Token::WhiteSpace(" "), Token::Decimal("0"), Token::WhiteSpace(" "), Token::Decimal("0"), Token::Eof])]
    #[case("1 2 3", vec![Token::Decimal("1"), Token::WhiteSpace(" "), Token::Decimal("2"), Token::WhiteSpace(" "), Token::Decimal("3"), Token::Eof])]
    #[case("19 28 37", vec![Token::Decimal("19"), Token::WhiteSpace(" "), Token::Decimal("28"), Token::WhiteSpace(" "), Token::Decimal("37"), Token::Eof])]
    #[case("0x0 0x0 0x0", vec![Token::Hexadecimal("0x0"), Token::WhiteSpace(" "), Token::Hexadecimal("0x0"), Token::WhiteSpace(" "), Token::Hexadecimal("0x0"), Token::Eof])]
    #[case("0x1 0X2 0x3", vec![Token::Hexadecimal("0x1"), Token::WhiteSpace(" "), Token::Hexadecimal("0X2"), Token::WhiteSpace(" "), Token::Hexadecimal("0x3"), Token::Eof])]
    #[case("0xAb 0XcD 0xEF", vec![Token::Hexadecimal("0xAb" ), Token::WhiteSpace(" "), Token::Hexadecimal("0XcD"), Token::WhiteSpace(" "), Token::Hexadecimal("0xEF"), Token::Eof])]
    #[case("0b111 0B1010 0b000", vec![Token::Binary("0b111" ), Token::WhiteSpace(" "), Token::Binary("0B1010"), Token::WhiteSpace(" "), Token::Binary("0b000"), Token::Eof])]
    #[case("0o123 077 090 0O70", vec![Token::Octal("0o123"), Token::WhiteSpace(" "), Token::Octal("077"), Token::WhiteSpace(" "), Token::Decimal("090"), Token::WhiteSpace(" "), Token::Octal("0O70"), Token::Eof])]
    #[case("0n 1n 9n", vec![Token::BigInt("0n"), Token::WhiteSpace(" "), Token::BigInt("1n"), Token::WhiteSpace(" "), Token::BigInt("9n"), Token::Eof])]
    #[case("0x0n 0Xfn 0xFn", vec![Token::BigHex("0x0n"), Token::WhiteSpace(" "), Token::BigHex("0Xfn"), Token::WhiteSpace(" "), Token::BigHex("0xFn"), Token::Eof])]
    #[case("0b1n 0B1n 0b101n", vec![Token::BigBin("0b1n"), Token::WhiteSpace(" "), Token::BigBin("0B1n"), Token::WhiteSpace(" "), Token::BigBin("0b101n"), Token::Eof])]
    #[case("0o0n 0O7n 0o77n", vec![Token::BigOct("0o0n"), Token::WhiteSpace(" "), Token::BigOct("0O7n"), Token::WhiteSpace(" "), Token::BigOct("0o77n"), Token::Eof])]
    #[case("00n 07n 08n", vec![Token::BigOct("00n"), Token::WhiteSpace(" "), Token::BigOct("07n"), Token::WhiteSpace(" "), Token::BigInt("08n"), Token::Eof])]
    #[case("0079 0077 0078", vec![Token::Decimal("0079"), Token::WhiteSpace(" "), Token::Octal("0077"), Token::WhiteSpace(" "), Token::Decimal("0078"), Token::Eof])]
    #[case("0079n 0077n 0078n", vec![Token::BigInt("0079n"), Token::WhiteSpace(" "), Token::BigOct("0077n"), Token::WhiteSpace(" "), Token::BigInt("0078n"), Token::Eof])]
    fn numbers(#[case] s: &str, #[case] expected: Vec<Token>) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens);
    }
}
