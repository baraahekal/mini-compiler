use crate::token::{Token, TokenType, TokenGlobal};
use serde::Serialize;

pub enum ExprNode {
    Add(Box<ExprNode>, Box<ExprNode>),
    IntLiteral(i32),
    FloatLiteral(f32),
    CharLiteral(char),
    StringLiteral(String),
    BoolLiteral(bool),
    Variable(String),
    IntIdentifier(String),
    FloatIdentifier(String),
    BoolIdentifier(String),
    StringIdentifier(String),
    DoubleIdentifier(String),
    CharIdentifier(String),
}

pub enum StmtNode {
    Assignment(String, ExprNode),
}

pub struct ProgramNode {
    pub statements: Vec<StmtNode>,
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    message: String,
    line: usize,
    column: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse_program(&mut self) -> Result<ProgramNode, Vec<ErrorMessage>> {
        println!("current parse_prgrm: {:?}", self.current);

        let mut statements = Vec::new();
        let mut errors = Vec::new();

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => errors.push(e),
            }

        if errors.is_empty() {
            Ok(ProgramNode { statements })
        } else {
            Err(errors)
        }
    }

    fn parse_statement(&mut self) -> Result<StmtNode, ErrorMessage> {
        println!("current parse_stmt: {:?}", self.current);
        let token = self.tokens[self.current].clone();

        match token.token_global {
            TokenGlobal::Identifier => {
                match token.token_type {
                    TokenType::Int | TokenType::Float | TokenType::Bool | TokenType::String | TokenType::Double | TokenType::Char => {
                        let variable_token = match self.advance() {
                            Ok(token) => token,
                            Err(e) => return Err(self.error(e)),
                        };
                        if variable_token.token_global == TokenGlobal::Variable {
                            let variable_name = variable_token.lexeme;
                            println!("variable_name: {:?}", variable_name);
                            if self.match_token(TokenType::Assignment) {
                                println!("current parse_stmt: {:?}", self.current);
                                if self.current < self.tokens.len() {
                                    self.current += 1;
                                }
                                let expr = self.parse_expression()?;
                                Ok(StmtNode::Assignment(variable_name, expr))
                            } else {
                                Err(self.error("Expected an assignment operator"))
                            }
                        } else {
                            Err(self.error("Expected a variable"))
                        }
                    },
                    _ => Err(self.error("Expected a type identifier")),
                }
            },
            _ => Err(self.error("Expected an identifier")),
        }
    }

    fn parse_expression(&mut self) -> Result<ExprNode, ErrorMessage> {
        let token = self.tokens[self.current].clone();

        match token.token_global {
            TokenGlobal::Literal => {
                let expr_node = match token.token_type {
                    TokenType::IntegerLiteral => {
                        println!("token lexeme: {:?}", token.lexeme);
                        let value = token.lexeme.parse::<i32>().map_err(|_| self.error("Invalid integer literal"))?;
                        ExprNode::IntLiteral(value)
                    },
                    TokenType::FloatingLiteral => {
                        let value = token.lexeme.parse::<f32>().map_err(|_| self.error("Invalid floating literal"))?;
                        ExprNode::FloatLiteral(value)
                    },
                    TokenType::CharacterLiteral => {
                        let value = token.lexeme.chars().next().ok_or_else(|| self.error("Invalid character literal"))?;
                        ExprNode::CharLiteral(value)
                    },
                    TokenType::StringLiteral => {
                        ExprNode::StringLiteral(token.lexeme.clone())
                    },
                    TokenType::BooleanLiteral => {
                        let value = token.lexeme.parse::<bool>().map_err(|_| self.error("Invalid boolean literal"))?;
                        ExprNode::BoolLiteral(value)
                    },
                    _ => return Err(self.error("Expected a literal")),
                };
                println!("current pos: {:?}", self.current);
                if self.match_token(TokenType::Semicolon) {
                    Ok(expr_node)
                } else {
                    Err(self.error("Expected a semicolon after a literal"))
                }
            },
            TokenGlobal::Identifier => {
                match token.token_type {
                    TokenType::Int => Ok(ExprNode::IntIdentifier(token.lexeme)),
                    TokenType::Float => Ok(ExprNode::FloatIdentifier(token.lexeme)),
                    TokenType::Bool => Ok(ExprNode::BoolIdentifier(token.lexeme)),
                    TokenType::String => Ok(ExprNode::StringIdentifier(token.lexeme)),
                    TokenType::Double => Ok(ExprNode::DoubleIdentifier(token.lexeme)),
                    TokenType::Char => Ok(ExprNode::CharIdentifier(token.lexeme)),
                    _ => Err(self.error("Unknown identifier")),
                }
            },
            _ => Err(self.error("Expected a literal or identifier")),
        }
    }

    fn is_at_end(&self) -> bool {
        // println!("current is_at_end: {:?}", self.current);
        // println!("tokens len: {:?}", self.tokens.len());
        self.current >= self.tokens.len()
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        self.current += 1;
        // println!("current match_token: {:?}", self.current);
        if self.is_at_end() {
            println!("is_at_end: {:?}", self.is_at_end());
            false
        } else {
            if self.tokens[self.current].token_type == token_type {
                true
            } else {
                false
            }
        }
    }

    fn advance(&mut self) -> Result<Token, &'static str> {
        if self.current < self.tokens.len() {
            self.current += 1;
            let token = self.tokens[self.current].clone();
            Ok(token)
        } else {
            Err("No more tokens")
        }
    }

    fn error(&self, message: &str) -> ErrorMessage {
        let token = &self.tokens[self.current - 1];
        let error_message = ErrorMessage {
            message: format!("Syntax Error: {}", message),
            line: token.line,
            column: token.column,
        };
        error_message
    }
}