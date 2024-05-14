use std::collections::{HashMap, HashSet};
use std::env::var;
use crate::token::{Token, TokenType, TokenGlobal};
use serde::Serialize;
use warp::Filter;


#[derive(Debug)]
pub enum ExprNode {
    Add(Box<ExprNode>, Box<ExprNode>),
    LessThan(Box<ExprNode>, Box<ExprNode>),
    Binary(Box<ExprNode>, TokenType, Box<ExprNode>),
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
    ForLoop(Box<StmtNode>, Box<ExprNode>, Box<StmtNode>),
    IfStatement(ExprNode, Vec<StmtNode>, Option<Vec<StmtNode>>),
}

pub struct ProgramNode {
    pub statements: Vec<StmtNode>,
}

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    message_type: String,
    message: String,
    line: usize,
    column: usize,
}


pub struct Parser {
    tokens: Vec<Token>,
    declared_variables: HashMap<String, TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            declared_variables: HashMap::new(),
        }
    }

    pub fn parse_program(&mut self) -> Result<ProgramNode, Vec<ErrorMessage>> {
        // println!("current parse_prgrm: {:?}", self.current);

        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                    self.current += 1;
                },
                Err(e) => {
                    errors.push(e);

                    if (self.current) < self.tokens.len() {
                        let cur_line = self.tokens[self.current].line;
                        while !self.is_at_end() && self.tokens[self.current].line == cur_line {
                            self.current += 1;
                        }
                    }
                },
            }
        }
        println!("errors total: {:?}", errors);

        if errors.is_empty() {
            Ok(ProgramNode { statements })
        } else {
            Err(errors)
        }
    }

    fn parse_statement(&mut self) -> Result<StmtNode, ErrorMessage> {
        let token = self.tokens[self.current].clone();

        match token.token_global {
            TokenGlobal::Identifier => {
                println!("entered identifier");
                match token.token_type {
                    TokenType::Int | TokenType::Float | TokenType::Bool | TokenType::String | TokenType::Double | TokenType::Char => self.parse_declaration(),
                    _ => Err(self.error("Expected a type identifier", "Error")),
                }
            },
            TokenGlobal::Variable => self.parse_assignment(),
            TokenGlobal::ReservedWord => {
                match token.token_type {
                    // TokenType::Print => self.parse_print(),
                    TokenType::If => self.parse_if_statement(),
                    // TokenType::While => self.parse_while_loop(),
                    // TokenType::OpenBrace => self.parse_block(),
                    _ => Err(self.error("Unexpected reserved word in statement", "Error")),
                }
            },
            _ => Err(self.error("Expected an identifier", "Error")),
        }
    }

    fn parse_declaration(&mut self) -> Result<StmtNode, ErrorMessage> {
        let variable_type = self.tokens[self.current].token_type.clone();
        println!("variable_type: {:?}", variable_type);

        self.current += 1; // Consume the type identifier

        let variable_token = self.tokens[self.current].clone();
        if variable_token.token_global != TokenGlobal::Variable {
            return Err(self.error("Expected a variable", "Error"));
        }
        let variable_name = variable_token.lexeme;
        println!("variable_name: {:?}", variable_name);

        self.current += 1; // Consume the variable

        let expr = if self.match_token(TokenType::Assignment).is_some() {
            println!("Matched assignment");
            self.parse_expression()?
        } else {
            ExprNode::IntLiteral(0) // Use a default value
        };

        println!("current parse_declaration: {:?}", self.current);
        if self.match_token(TokenType::Semicolon).is_none() {
            return Err(self.error("Expected a semicolon", "Error"));
        }



        self.declared_variables.insert(variable_name.clone(), variable_type);
        Ok(StmtNode::Assignment(variable_name, expr))
    }

    fn parse_assignment(&mut self) -> Result<StmtNode, ErrorMessage> {
        let variable_name = self.tokens[self.current].lexeme.clone();
        if !self.is_variable_declared(&variable_name) {
            return Err(self.error(&format!("Use of undeclared variable '{}'", variable_name), "Error"));
        }
        self.current += 1; // Consume the variable

        if !self.match_token(TokenType::Assignment).is_none() {
            return Err(self.error("Expected an assignment operator", "Error"));
        }

        let expr = self.parse_expression()?;

        if !self.match_token(TokenType::Semicolon).is_none() {
            return Err(self.error("Expected a semicolon", "Error"));
        }

        Ok(StmtNode::Assignment(variable_name, expr))
    }

    fn parse_expression(&mut self) -> Result<ExprNode, ErrorMessage> {
        let mut expr = self.parse_term()?;

        while let Some(operator) = self.match_token(TokenType::Plus).or(self.match_token(TokenType::Minus)) {
            let right = self.parse_term()?;
            expr = ExprNode::Binary(Box::new(expr), operator, Box::new(right));
        }
        println!("Valid expr");
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<ExprNode, ErrorMessage> {
        let mut expr = self.parse_factor()?;

        while let Some(operator) = self.match_token(TokenType::Multiply).or(self.match_token(TokenType::Divide)) {
            let right = self.parse_factor()?;
            expr = ExprNode::Binary(Box::new(expr), operator, Box::new(right));
        }

        // println!("current parse_term: {:?}", self.current);

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<ExprNode, ErrorMessage> {
        let token = self.tokens[self.current].clone();
        match &token.token_type {
            TokenType::OpenParen => {
                self.current += 1; // Consume the OpenParen token
                let expr = self.parse_expression()?;
                if let TokenType::CloseParen = self.tokens[self.current].token_type {
                    self.current += 1; // Consume the CloseParen token
                    Ok(expr)
                } else {
                    Err(self.error("Expected a closing parenthesis", "Error"))
                }
            },
            TokenType::IntegerLiteral => {
                match token.lexeme.parse::<i32>() {
                    Ok(value) => {
                        self.current += 1; // Consume the literal token
                        Ok(ExprNode::IntLiteral(value))
                    },
                    Err(_) => Err(self.error("Expected a valid integer", "Error")),
                }
            },
            TokenType::FloatingLiteral => {
                match token.lexeme.parse::<f32>() {
                    Ok(value) => {
                        self.current += 1; // Consume the literal token
                        Ok(ExprNode::FloatLiteral(value))
                    },
                    Err(_) => Err(self.error("Expected a valid float", "Error")),
                }
            },
            TokenType::CharacterLiteral => {
                if token.lexeme.len() == 1 {
                    self.current += 1; // Consume the literal token
                    Ok(ExprNode::CharLiteral(token.lexeme.chars().next().unwrap()))
                } else {
                    Err(self.error("Expected a valid character", "Error"))
                }
            },
            TokenType::StringLiteral => {
                self.current += 1; // Consume the literal token
                Ok(ExprNode::StringLiteral(token.lexeme.clone()))
            },
            TokenType::BooleanLiteral => {
                match token.lexeme.parse::<bool>() {
                    Ok(value) => {
                        self.current += 1; // Consume the literal token
                        Ok(ExprNode::BoolLiteral(value))
                    },
                    Err(_) => Err(self.error("Expected a valid boolean", "Error")),
                }
            },
            TokenType::Variable => {
                self.current += 1; // Consume the variable token
                Ok(ExprNode::Variable(token.lexeme.clone()))
            },
            _ => Err(self.error("Expected a number, variable, or expression", "Error")),
        }
    }

    fn parse_comparison(&mut self) -> Result<ExprNode, ErrorMessage> {
        let left = self.parse_expression()?;

        let operator = match &self.tokens[self.current].token_type {
            TokenType::Equal => TokenType::Equal,
            TokenType::NotEqual => TokenType::NotEqual,
            TokenType::LessThan => TokenType::LessThan,
            TokenType::LessThanOrEqual => TokenType::LessThanOrEqual,
            TokenType::GreaterThan => TokenType::GreaterThan,
            TokenType::GreaterThanOrEqual => TokenType::GreaterThanOrEqual,
            _ => return Err(self.error("Expected a comparison operator", "Error")),
        };

        self.current += 1; // Consume the operator

        let right = self.parse_expression()?;

        Ok(ExprNode::Binary(Box::new(left), operator, Box::new(right)))
    }

    fn parse_condition(&mut self) -> Result<(ExprNode), ErrorMessage> {
        let current = self.current;
        match self.parse_comparison() {
            Ok(comparison) => {
                if let ExprNode::Binary(left, operator, right) = &comparison {
                    if let (ExprNode::IntLiteral(l_val), ExprNode::IntLiteral(r_val)) = (&**left, &**right) {
                        match operator {
                            TokenType::GreaterThan if l_val > r_val => (),
                            TokenType::LessThan if l_val < r_val => (),
                            TokenType::Equal if l_val == r_val => (),
                            TokenType::NotEqual if l_val != r_val => (),
                            TokenType::GreaterThanOrEqual if l_val >= r_val => (),
                            TokenType::LessThanOrEqual if l_val <= r_val => (),
                            _ => return Err(self.error("Warning: This condition is always false", "Warning")),
                        }
                    }
                }
                Ok(comparison)
            },
            Err(_) => {
                self.current = current;
                let expr = self.parse_expression()?;
                match &expr {
                    ExprNode::BoolLiteral(value) => {
                        if !value {
                            return Err(self.error("Warning: This condition is always false", "Warning"));
                        }
                    },
                    ExprNode::IntLiteral(value) => {
                        if *value == 0 {
                            return Err(self.error("Warning: This condition is always false", "Warning"));
                        }
                    },
                    _ => (),
                }
                Ok(expr)
            },
        }
    }

    fn parse_if_statement(&mut self) -> Result<StmtNode, ErrorMessage> {
        if self.match_token(TokenType::If).is_none() {
            return Err(self.error("Expected 'if'", "Error"));
        }

        if self.match_token(TokenType::OpenParen).is_none() {
            return Err(self.error("Expected '('", "Error"));
        }

        let condition = self.parse_condition()?;

        if self.match_token(TokenType::CloseParen).is_none() {
            return Err(self.error("Expected ')'", "Error"));
        }

        if self.match_token(TokenType::OpenBrace).is_none() {
            return Err(self.error("Expected '{'", "Error"));
        }

        let mut then_branch = Vec::new();
        while let Ok(stmt) = self.parse_statement() {
            then_branch.push(stmt);
            if self.match_token(TokenType::CloseBrace).is_some() {
                break;
            }
        }

        let else_branch = if self.match_token(TokenType::Else).is_some() {
            if self.match_token(TokenType::OpenBrace).is_none() {
                return Err(self.error("Expected '{'", "Error"));
            }

            let mut else_statements = Vec::new();
            while let Ok(stmt) = self.parse_statement() {
                else_statements.push(stmt);
                if self.match_token(TokenType::CloseBrace).is_some() {
                    break;
                }
            }
            Some(else_statements)
        } else {
            None
        };

        Ok(StmtNode::IfStatement(condition, then_branch, else_branch))
    }

    fn parse_for_statement(&mut self) -> Result<StmtNode, ErrorMessage> {
        if self.match_token(TokenType::OpenParen).is_some() {
            self.current += 1; // Consume the OpenParen token
            println!("current for: {:?}", self.tokens[self.current]);
            let initialization = self.parse_statement()?;

            self.current += 1; // Consume the semicolon
            println!("current for: {:?}", self.tokens[self.current]);
            let condition = self.parse_condition()?;
            println!("Good condition: {:?}", condition);

            self.current += 1; // Consume the semicolon
            println!("current for: {:?}", self.tokens[self.current]);
            let increment = self.parse_statement()?;

            Ok(StmtNode::ForLoop(Box::new(initialization), Box::new(condition), Box::new(increment)))
        } else {
            Err(self.error("Expected an opening parenthesis", "Error"))
        }
    }

    fn is_at_end(&self) -> bool {
        // println!("current is_at_end: {:?}", self.current);
        // println!("tokens len: {:?}", self.tokens.len());
        self.current >= self.tokens.len()
    }

    fn match_token(&mut self, token_type: TokenType) -> Option<TokenType> {
        if self.is_at_end() {
            None
        } else if self.tokens[self.current].token_type == token_type {
            self.current += 1;
            Some(token_type)
        } else {
            None
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

    fn is_variable_declared(&self, variable_name: &str) -> bool {
        self.declared_variables.contains_key(variable_name)
    }

    fn error(&self, message: &str, message_type_: &str) -> ErrorMessage {
        println!("current error: {:?}", message);
        let token = &self.tokens[if self.current >= 1 { self.current - 1 } else { self.current }];
        let error_message = ErrorMessage {
            message_type: message_type_.to_string(),
            message: format!("{}", message),
            line: token.line,
            column: token.column,
        };
        error_message
    }

}
