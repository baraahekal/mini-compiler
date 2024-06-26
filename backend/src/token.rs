use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    // Reserved Words
    Break,
    Case,
    Do,
    Else,
    For,
    If,
    Return,
    While,
    Continue,
    Switch,

    // Identifiers
    Int,
    Float,
    Bool,
    String,
    Double,
    Char,
    Void,


    // Constants
    IntegerLiteral,
    FloatingLiteral,
    CharacterLiteral,
    StringLiteral,
    BooleanLiteral,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    MinusMinus,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    Assignment,
    PlusAssignment,
    MinusAssignment,
    MultiplyAssignment,
    DivideAssignment,
    ModuloAssignment,

    // Punctuation
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Semicolon,
    Colon,
    Exclamation,
    QuestionMark,
    DoubleColon,

    // Special tokens
    Comment,
    Variable,
    List,
    Error,
    PlusPlus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenGlobal {
    Identifier,
    Literal,
    Symbol,
    ReservedWord,
    Variable,
    List,
    Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token_global: TokenGlobal,
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub original_line: usize,
    pub original_column: usize,
}
