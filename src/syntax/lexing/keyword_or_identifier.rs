use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct KeyworkOrIdentifier;

impl LexerState for KeyworkOrIdentifier {}

impl<'i, 'o> Lexer<'i, KeyworkOrIdentifier>
where
    'i: 'o,
{
    #[must_use]
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        let mut end = start;
        while let Some(&(_, c2)) = self.chars.peek()
            && (c2.is_alphanumeric() || c2 == '_')
        {
            end += 1;
            self.next_char();
        }
        let s = &self.input[start..end];
        let token = match s {
            "null" => Token::Null(start),
            "true" => Token::True(start),
            "false" => Token::False(start),
            "break" => Token::Break(start),
            "case" => Token::Case(start),
            "catch" => Token::Catch(start),
            "class" => Token::Class(start),
            "const" => Token::Const(start),
            "continue" => Token::Continue(start),
            "debugger" => Token::Debugger(start),
            "default" => Token::Default(start),
            "delete" => Token::Delete(start),
            "do" => Token::Do(start),
            "else" => Token::Else(start),
            "export" => Token::Export(start),
            "extends" => Token::Extends(start),
            "finally" => Token::Finally(start),
            "for" => Token::For(start),
            "function" => Token::Function(start),
            "if" => Token::If(start),
            "import" => Token::Import(start),
            "in" => Token::In(start),
            "instanceof" => Token::Instanceof(start),
            "new" => Token::New(start),
            "return" => Token::Return(start),
            "super" => Token::Super(start),
            "switch" => Token::Switch(start),
            "this" => Token::This(start),
            "throw" => Token::Throw(start),
            "try" => Token::Try(start),
            "typeof" => Token::Typeof(start),
            "var" => Token::Var(start),
            "void" => Token::Void(start),
            "while" => Token::While(start),
            "with" => Token::With(start),
            "let" => Token::Let(start),
            "static" => Token::Static(start),
            "yield" => Token::Yield(start),
            "await" => Token::Await(start),
            "async" => Token::Async(start),
            "arguments" => Token::Arguments(start),
            "as" => Token::As(start),
            "eval" => Token::Eval(start),
            "from" => Token::From(start),
            "get" => Token::Get(start),
            "of" => Token::Of(start),
            "set" => Token::Set(start),
            "enum" => Token::Enum(start),
            "implements" => Token::Implements(start),
            "interface" => Token::Interface(start),
            "package" => Token::Package(start),
            "private" => Token::Private(start),
            "protected" => Token::Protected(start),
            "public" => Token::Public(start),
            _ => Token::Identifier(s),
        };
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("null", Token::Null(0))]
    #[case("true", Token::True(0))]
    #[case("false", Token::False(0))]
    #[case("break", Token::Break(0))]
    #[case("case", Token::Case(0))]
    #[case("catch", Token::Catch(0))]
    #[case("class", Token::Class(0))]
    #[case("const", Token::Const(0))]
    #[case("continue", Token::Continue(0))]
    #[case("debugger", Token::Debugger(0))]
    #[case("default", Token::Default(0))]
    #[case("delete", Token::Delete(0))]
    #[case("do", Token::Do(0))]
    #[case("else", Token::Else(0))]
    #[case("export", Token::Export(0))]
    #[case("extends", Token::Extends(0))]
    #[case("finally", Token::Finally(0))]
    #[case("for", Token::For(0))]
    #[case("function", Token::Function(0))]
    #[case("if", Token::If(0))]
    #[case("import", Token::Import(0))]
    #[case("in", Token::In(0))]
    #[case("instanceof", Token::Instanceof(0))]
    #[case("new", Token::New(0))]
    #[case("return", Token::Return(0))]
    #[case("super", Token::Super(0))]
    #[case("switch", Token::Switch(0))]
    #[case("this", Token::This(0))]
    #[case("throw", Token::Throw(0))]
    #[case("try", Token::Try(0))]
    #[case("typeof", Token::Typeof(0))]
    #[case("var", Token::Var(0))]
    #[case("void", Token::Void(0))]
    #[case("while", Token::While(0))]
    #[case("with", Token::With(0))]
    #[case("let", Token::Let(0))]
    #[case("static", Token::Static(0))]
    #[case("yield", Token::Yield(0))]
    #[case("await", Token::Await(0))]
    #[case("async", Token::Async(0))]
    #[case("arguments", Token::Arguments(0))]
    #[case("as", Token::As(0))]
    #[case("eval", Token::Eval(0))]
    #[case("from", Token::From(0))]
    #[case("get", Token::Get(0))]
    #[case("of", Token::Of(0))]
    #[case("set", Token::Set(0))]
    #[case("enum", Token::Enum(0))]
    #[case("implements", Token::Implements(0))]
    #[case("interface", Token::Interface(0))]
    #[case("package", Token::Package(0))]
    #[case("private", Token::Private(0))]
    #[case("protected", Token::Protected(0))]
    #[case("public", Token::Public(0))]
    fn keyword(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[rstest]
    #[case("x")]
    #[case("x2")]
    #[case("myvar")]
    #[case("myvar2")]
    #[case("a1b2c3")]
    #[case("my_var")]
    #[case("my_other_var")]
    #[case("_x")]
    #[case("_1")]
    fn single_identifier(#[case] s: &str) {
        let tokens = lex_all(s);
        let expected = Token::Identifier(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[rstest]
    #[case("a b c", vec![Token::Identifier("a"), Token::WhiteSpace(" "), Token::Identifier("b"), Token::WhiteSpace(" "), Token::Identifier("c"), Token::Eof])]
    #[case("a1 b9 c0", vec![Token::Identifier("a1"), Token::WhiteSpace(" "), Token::Identifier("b9"), Token::WhiteSpace(" "), Token::Identifier("c0"), Token::Eof])]
    #[case("_X _y _Z", vec![Token::Identifier("_X"), Token::WhiteSpace(" "), Token::Identifier("_y"), Token::WhiteSpace(" "), Token::Identifier("_Z"), Token::Eof])]
    #[case("public class Foo", vec![Token::Public(0), Token::WhiteSpace(" "), Token::Class(7), Token::WhiteSpace(" "), Token::Identifier("Foo"), Token::Eof])]
    fn identifiers(#[case] s: &str, #[case] expected: Vec<Token>) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens);
    }
}
