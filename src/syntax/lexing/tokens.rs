/// Each token holds either a &str to the input string
/// or a usize index into the input string if the token len is constant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Identifier(&'a str),

    // literals
    Null(usize),
    True(usize),
    False(usize),
    Decimal(&'a str),
    /// beN/bEN where b=base, N=exponent
    Exponential(&'a str),
    /// [0-9]...e...n
    BigExpDec(&'a str),
    /// ends with n
    BigInt(&'a str),
    /// starts with 0b/0B
    Binary(&'a str),
    /// 0b...n
    BigBin(&'a str),
    /// 0b...e...
    ExpBin(&'a str),
    /// 0b...e...n
    BigExpBin(&'a str),
    /// starts with 0/0o/0O
    Octal(&'a str),
    /// 0o...n
    BigOct(&'a str),
    /// 0o...e...
    ExpOct(&'a str),
    /// 0o...e...n
    BigExpOct(&'a str),
    /// starts with 0x/0X
    Hexadecimal(&'a str),
    /// 0x...n
    BigHex(&'a str),
    String(&'a str),
    Regex(&'a str),
    Template(&'a str),

    // keywords
    Break(usize),
    Case(usize),
    Catch(usize),
    Class(usize),
    Const(usize),
    Continue(usize),
    Debugger(usize),
    Default(usize),
    Delete(usize),
    Do(usize),
    Else(usize),
    Export(usize),
    Extends(usize),
    Finally(usize),
    For(usize),
    Function(usize),
    If(usize),
    Import(usize),
    In(usize),
    Instanceof(usize),
    New(usize),
    Return(usize),
    Super(usize),
    Switch(usize),
    This(usize),
    Throw(usize),
    Try(usize),
    Typeof(usize),
    Var(usize),
    Void(usize),
    While(usize),
    With(usize),
    Let(usize),
    Static(usize),
    Yield(usize),
    Await(usize),
    Async(usize),
    Arguments(usize),
    As(usize),
    Eval(usize),
    From(usize),
    Get(usize),
    Of(usize),
    Set(usize),
    // future reserved
    Enum(usize),
    Implements(usize),
    Interface(usize),
    Package(usize),
    Private(usize),
    Protected(usize),
    Public(usize),

    // punctuators
    LeftParen(usize),    // (
    RightParen(usize),   // )
    LeftBracket(usize),  // [
    RightBracket(usize), // ]
    LeftBrace(usize),    // {
    RightBrace(usize),   // }

    Bang(usize),            // !
    BangEqual(usize),       // !=
    BangEqual2(usize),      // !==
    Equal(usize),           // =
    Equal2(usize),          // ==
    Equal3(usize),          // ===
    Greater(usize),         // >
    Greater2(usize),        // >>
    Greater3(usize),        // >>>
    GreaterEqual(usize),    // >=
    Greater2Equal(usize),   // >>=
    Greater3Equal(usize),   // >>>=
    Less(usize),            // <
    Less2(usize),           // <<
    LessEqual(usize),       // <=
    Less2Equal(usize),      // <<=
    Plus(usize),            // +
    Plus2(usize),           // ++
    PlusEqual(usize),       // +=
    Minus(usize),           // -
    Minus2(usize),          // --
    MinusEqual(usize),      // -=
    Star(usize),            // *
    Star2(usize),           // **
    StarEqual(usize),       // *=
    Star2Equal(usize),      // **=
    Div(usize),             // /
    DivEqual(usize),        // /=
    Percent(usize),         // %
    PercentEqual(usize),    // %=
    Ampersand(usize),       // &
    Ampersand2(usize),      // &&
    AmpersandEqual(usize),  // &=
    Ampersand2Equal(usize), // &&=
    Pipe(usize),            // |
    Pipe2(usize),           // ||
    PipeEqual(usize),       // |=
    Pipe2Equal(usize),      // ||=
    Caret(usize),           // ^
    Tilde(usize),           // ~
    Dot(usize),             // .
    Dot2(usize),            // ..
    Dot3(usize),            // ...
    Comma(usize),           // ,
    Colon(usize),           // :
    SemiColon(usize),       // ;
    Question(usize),        // ?
    Question2(usize),       // ??
    Question2Equal(usize),  // ??=
    QuestionDot(usize),     // ?.
    Arrow(usize),           // =>
    BackSlash(usize),       // \

    WhiteSpace(&'a str),
    NewLine(&'a str),

    Invalid(&'a str),
    Eof,
}

impl Token<'_> {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub const fn len(&self) -> usize {
        match self {
            Token::Identifier(s)
            | Token::Decimal(s)
            | Token::Exponential(s)
            | Token::BigExpDec(s)
            | Token::BigInt(s)
            | Token::Binary(s)
            | Token::BigBin(s)
            | Token::ExpBin(s)
            | Token::BigExpBin(s)
            | Token::Octal(s)
            | Token::BigOct(s)
            | Token::ExpOct(s)
            | Token::BigExpOct(s)
            | Token::Hexadecimal(s)
            | Token::BigHex(s)
            | Token::String(s)
            | Token::Regex(s)
            | Token::Template(s)
            | Token::WhiteSpace(s)
            | Token::NewLine(s)
            | Token::Invalid(s) => s.len(),

            Token::LeftParen(_)
            | Token::RightParen(_)
            | Token::LeftBracket(_)
            | Token::RightBracket(_)
            | Token::LeftBrace(_)
            | Token::RightBrace(_)
            | Token::Bang(_)
            | Token::Equal(_)
            | Token::Greater(_)
            | Token::Less(_)
            | Token::Plus(_)
            | Token::Minus(_)
            | Token::Star(_)
            | Token::Div(_)
            | Token::Percent(_)
            | Token::Ampersand(_)
            | Token::Pipe(_)
            | Token::Caret(_)
            | Token::Tilde(_)
            | Token::Dot(_)
            | Token::Comma(_)
            | Token::Colon(_)
            | Token::SemiColon(_)
            | Token::Question(_) => 1,

            Token::Do(_)
            | Token::If(_)
            | Token::In(_)
            | Token::As(_)
            | Token::Of(_)
            | Token::BangEqual(_)
            | Token::Equal2(_)
            | Token::Greater2(_)
            | Token::GreaterEqual(_)
            | Token::Less2(_)
            | Token::LessEqual(_)
            | Token::Plus2(_)
            | Token::PlusEqual(_)
            | Token::Minus2(_)
            | Token::MinusEqual(_)
            | Token::Star2(_)
            | Token::StarEqual(_)
            | Token::DivEqual(_)
            | Token::PercentEqual(_)
            | Token::Ampersand2(_)
            | Token::AmpersandEqual(_)
            | Token::Pipe2(_)
            | Token::PipeEqual(_)
            | Token::Dot2(_)
            | Token::Question2(_)
            | Token::QuestionDot(_)
            | Token::Arrow(_) => 2,

            Token::For(_)
            | Token::New(_)
            | Token::Try(_)
            | Token::Var(_)
            | Token::Let(_)
            | Token::Get(_)
            | Token::Set(_)
            | Token::BangEqual2(_)
            | Token::Equal3(_)
            | Token::Greater3(_)
            | Token::Greater2Equal(_)
            | Token::Less2Equal(_)
            | Token::Star2Equal(_)
            | Token::Ampersand2Equal(_)
            | Token::Pipe2Equal(_)
            | Token::Dot3(_)
            | Token::Question2Equal(_)
            | Token::BackSlash(_) => 3,

            Token::Null(_)
            | Token::True(_)
            | Token::Case(_)
            | Token::Else(_)
            | Token::This(_)
            | Token::Void(_)
            | Token::With(_)
            | Token::Eval(_)
            | Token::From(_)
            | Token::Enum(_)
            | Token::Greater3Equal(_) => 4,

            Token::False(_)
            | Token::Break(_)
            | Token::Catch(_)
            | Token::Class(_)
            | Token::Const(_)
            | Token::Super(_)
            | Token::Throw(_)
            | Token::While(_)
            | Token::Yield(_)
            | Token::Await(_)
            | Token::Async(_) => 5,

            Token::Delete(_)
            | Token::Export(_)
            | Token::Import(_)
            | Token::Return(_)
            | Token::Switch(_)
            | Token::Typeof(_)
            | Token::Static(_)
            | Token::Public(_) => 6,

            Token::Default(_)
            | Token::Extends(_)
            | Token::Finally(_)
            | Token::Package(_)
            | Token::Private(_) => 7,

            Token::Continue(_) | Token::Debugger(_) | Token::Function(_) => 8,

            Token::Arguments(_) | Token::Interface(_) | Token::Protected(_) => 9,

            Token::Instanceof(_) | Token::Implements(_) => 10,

            Token::Eof => 0,
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
