use eq_float::F64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Identifier(&'a str),

    // literals
    Null,
    True,
    False,
    Decimal(F64),
    Exponential { base: F64, exp: i64 }, // beN/bEN where b=base, N=exponent
    Binary(i64),                         // starts with 0b/0B
    Octal(i64),                          // starts with 0/0o/0O
    Hexadecimal(i64),                    // starts with 0x/0X
    BigInt(&'a str),                     // ends with n
    String(&'a str),
    Regex(&'a str),
    Template(&'a str),

    // keywords
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Let,
    Static,
    Yield,
    Await,
    Async,
    Arguments,
    As,
    Eval,
    From,
    Get,
    Of,
    Set,
    // future reserved
    Enum,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,

    // punctuators
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    Bang,            // !
    BangEqual,       // !=
    BangEqual2,      // !==
    Equal,           // =
    Equal2,          // ==
    Equal3,          // ===
    Greater,         // >
    Greater2,        // >>
    Greater3,        // >>>
    GreaterEqual,    // >=
    Greater2Equal,   // >>=
    Greater3Equal,   // >>>=
    Less,            // <
    Less2,           // <<
    LessEqual,       // <=
    Less2Equal,      // <<=
    Plus,            // +
    Plus2,           // ++
    PlusEqual,       // +=
    Minus,           // -
    Minus2,          // --
    MinusEqual,      // -=
    Star,            // *
    Star2,           // **
    StarEqual,       // *=
    Star2Equal,      // **=
    Div,             // /
    DivEqual,        // /=
    Percent,         // %
    PercentEqual,    // %=
    Ampersand,       // &
    Ampersand2,      // &&
    AmpersandEqual,  // &=
    Ampersand2Equal, // &&=
    Pipe,            // |
    Pipe2,           // ||
    PipeEqual,       // |=
    Pipe2Equal,      // ||=
    Caret,           // ^
    Tilde,           // ~
    Dot,             // .
    Dot3,            // ...
    Comma,           // ,
    Colon,           // :
    SemiColon,       // ;
    Question,        // ?
    Question2,       // ??
    Question2Equal,  // ??=
    QuestionDot,     // ?.
    Arrow,           // =>
    BackSlash,       // \

    WhiteSpace,
    NewLine,
    Eof,
}
