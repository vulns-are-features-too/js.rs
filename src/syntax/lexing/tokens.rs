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
    Exponential(&'a str), /// beN/bEN where b=base, N=exponent
    BigExpDec(&'a str),   /// [0-9]...e...n
    BigInt(&'a str),      /// ends with n
    Binary(&'a str),      /// starts with 0b/0B
    BigBin(&'a str),      /// 0b...n
    ExpBin(&'a str),      /// 0b...e...
    BigExpBin(&'a str),   /// 0b...e...n
    Octal(&'a str),       /// starts with 0/0o/0O
    BigOct(&'a str),      /// 0o...n
    ExpOct(&'a str),      /// 0o...e...
    BigExpOct(&'a str),   /// 0o...e...n
    Hexadecimal(&'a str), /// starts with 0x/0X
    BigHex(&'a str),      /// 0x...n
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
