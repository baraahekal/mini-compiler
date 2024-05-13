use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    // Keywords
    Auto,
    Break,
    Case,
    Char,
    Continue,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    String,
    For,
    If,
    Int,
    Return,
    Void,
    While,
    Bool,

    // Identifiers
    Identifier,

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl Token {
    pub fn syntax_error(&self, message: &str) -> String {
        format!("Syntax Error: {} at line {}, column {}", message, self.line, self.column)
    }
}