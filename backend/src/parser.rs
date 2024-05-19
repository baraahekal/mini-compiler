use std::collections::{HashMap, HashSet};
use crate::token::{Token, TokenType, TokenGlobal};
use serde::Serialize;
use warp::Filter;


#[derive(Debug, Clone)]
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

use std::fmt;

impl fmt::Display for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprNode::Add(left, right) => write!(f, "Add({}, {})", left, right),
            ExprNode::LessThan(left, right) => write!(f, "LessThan({}, {})", left, right),
            ExprNode::Binary(left, _, right) => write!(f, "Binary({}, {})", left, right),
            ExprNode::IntLiteral(value) => write!(f, "{}", value),
            ExprNode::FloatLiteral(value) => write!(f, "{}", value),
            ExprNode::CharLiteral(value) => write!(f, "{}", value),
            ExprNode::StringLiteral(value) => write!(f, "{}", value),
            ExprNode::BoolLiteral(value) => write!(f, "{}", value),
            ExprNode::Variable(name) => write!(f, "{}", name),
            ExprNode::IntIdentifier(name) => write!(f, "{}", name),
            ExprNode::FloatIdentifier(name) => write!(f, "{}", name),
            ExprNode::BoolIdentifier(name) => write!(f, "{}", name),
            ExprNode::StringIdentifier(name) => write!(f, "{}", name),
            ExprNode::DoubleIdentifier(name) => write!(f, "{}", name),
            ExprNode::CharIdentifier(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug)]
pub enum StmtNode {
    Assignment(String, ExprNode),
    ForLoop(Box<StmtNode>, Box<ExprNode>, Box<StmtNode>, Box<StmtNode>),
    IfStatement(ExprNode, Box<StmtNode>, Option<Box<StmtNode>>),
    WhileLoop(Box<ExprNode>, Box<StmtNode>),
    DoWhileLoop(Box<ExprNode>, Box<StmtNode>),
    SwitchCase(Box<ExprNode>, Vec<(ExprNode, StmtNode)>),
    Block(Vec<StmtNode>),
}

pub struct ProgramNode {
    pub statements: Vec<StmtNode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorMessage {
    message_type: String,
    message: String,
    line: usize,
    column: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    declared_variables: HashMap<String, (TokenType, String)>,
    current: usize,
    errors: Vec<ErrorMessage>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            declared_variables: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Result<ProgramNode, Vec<ErrorMessage>> {
        // println!("current parse_prgrm: {:?}", self.current);

        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                    println!("current parse_program: {:?}", self.current);
                    self.current += 1;
                    println!("current parse_program: {:?}", self.current);
                },
                Err(e) => {
                    self.errors.push(e);

                    if (self.current) < self.tokens.len() {
                        let cur_line = self.tokens[self.current].line;
                        while !self.is_at_end() && self.tokens[self.current].line == cur_line {
                            self.current += 1;
                        }
                    }
                },
            }
        }
        println!("errors total: {:?}", self.errors);

        if self.errors.is_empty() {
            Ok(ProgramNode { statements })
        } else {
            Err(self.errors.clone())
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
            TokenGlobal::Variable => {
                println!("entered variable ");
                println!("current parse_statement: {:?}", self.current);
                self.parse_assignment()
            },
            TokenGlobal::ReservedWord => {
                match token.token_type {
                    TokenType::For => self.parse_for_statement(),
                    TokenType::If => self.parse_if_statement(),
                    TokenType::Do => self.parse_do_while_loop(),
                    TokenType::Switch => self.parse_switch_case(),
                    TokenType::While => self.parse_while_loop(),
                    TokenType::OpenBrace => self.parse_block(0),
                    TokenType::Case => {
                        self.current += 1;
                        Ok(StmtNode::Block(vec![]))
                    }
                    TokenType::Break => {
                        self.current += 1;
                        Ok(StmtNode::Block(vec![]))
                    },
                    _ => {
                        println!("reserved word: {:?}", token.token_type);
                        Err(self.error("Unexpected reserved word in statement", "Error"))
                    },
                }
            },
            TokenGlobal::Symbol => {
                match token.token_type {
                    TokenType::Semicolon => {
                        self.current += 1;
                        Ok(StmtNode::Block(vec![]))
                    },
                    TokenType::CloseBrace => {
                        println!("current ZZZZZZZZZZ: {:?}", self.current);
                        // self.current += 1;
                        Ok(StmtNode::Block(vec![]))
                    }
                    _ => Err(self.error("Unexpected symbol in statement", "Error")),
                }
            },
            _ => Err(self.error("Expected an identifier", "Error")),
        }
    }

    fn parse_block(&mut self, flag_do_while: i32) -> Result<StmtNode, ErrorMessage> {
        let mut statements = Vec::new();

        println!("ANAAAA DAKHAAAALT FOOOR LOOOOP {:?}", self.tokens[self.current].lexeme);
        if self.match_token(TokenType::OpenBrace).is_none() {
            println!("PLEEEEEEEEESEEEEE NNNNNNNOOOOOOOO open");

            self.errors.push(self.error("Expected '{'", "Error"));
        }

        if self.is_at_end() {
            return Err(self.error("Expected '}'", "Error"));
        }

        while let Ok(stmt) = self.parse_statement() {

            if (flag_do_while == 1 && self.tokens[self.current].lexeme == "while") {
                break;
            }

            // println!("current Gaaaatttt: {:?}  {:?}", self.current, self.tokens[self.current]);
            statements.push(stmt);
            if self.match_token(TokenType::Semicolon).is_some() {
                println!("HOOOOHHHHHHHH");
                self.current -= 1;
                break;
            }

            if self.match_token(TokenType::CloseBrace).is_some() {
                self.current -= 1;
                break;
            }
        }

        println!("current parse_block: {:?} {:?}", self.current, self.tokens[self.current]);
        if self.match_token(TokenType::CloseBrace).is_none() {
            println!("PLEEEEEEEEESEEEEE NNNNNNNOOOOOOOO close");
            return Err(self.error("Expected '}'", "Error"));
        }

        Ok(StmtNode::Block(statements))
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

        if self.is_variable_declared(&variable_name) {
            return Err(self.error(&format!("Variable '{}' already declared", variable_name), "Error"));
        }

        self.current += 1; // Consume the variable
        println!("current token: {:?}", self.tokens[self.current]);
        if self.match_token(TokenType::Assignment).is_none() {
            return Err(self.error("Expected an =", "Error"));
        }

        let expr = self.parse_expression()?;
        let expr_type = self.get_expr_type(&expr)?;

        if expr_type != variable_type {
            match (expr_type, variable_type.clone()) {
                (TokenType::Int, _) => return Err(self.error(&format!("Syntax Error: Cannot assign an Integer to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::Float, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a Float to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::Bool, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a Boolean to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::Char, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a Char to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::String, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a String to a variable of type '{:?}'", variable_type), "Error")),
                _ => (),
            }
        }

        if self.match_token(TokenType::Semicolon).is_none() {
            return Err(self.error("Expected a semicolon", "Error"));
        }
        self.current -= 1;
        println!("after declaration: {:?}", self.current);


        self.declared_variables.insert(variable_name.clone(), (variable_type.clone(), expr.to_string()));
        Ok(StmtNode::Assignment(variable_name, expr))
    }

    fn parse_assignment(&mut self) -> Result<StmtNode, ErrorMessage> {
        let variable_name_before = self.tokens[self.current].lexeme.clone();

        let variable_type = self.get_variable_type(&variable_name_before)?;

        self.current += 1; // Consume the variable

        let operator = self.tokens[self.current].token_type.clone();
        println!("{:?} enta meeeen:   ", operator);
        self.current += 1; // Consume the operator
        println!("current parse_assignment: {:?}", self.tokens[self.current]);

        let right = match operator {
            TokenType::Plus => {
                if self.tokens[self.current].token_type == TokenType::Plus {
                    self.current += 1; // Consume the second '+'
                    let updated_value = match self.declared_variables.get(&variable_name_before) {
                        Some(value) => match value.1.parse::<i32>() {
                            Ok(parsed_value) => parsed_value + 1,
                            Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                        },
                        None => return Err(self.error("Variable not found", "Error")),
                    };
                    self.update_variable_value(variable_name_before.clone(), updated_value.to_string())?;
                    ExprNode::IntLiteral(1) // Handle i++
                } else if self.tokens[self.current].token_type == TokenType::Assignment {
                    self.current += 1; // Consume the '='
                    let increment_value = match self.tokens[self.current].lexeme.parse::<i32>() {
                        Ok(value) => value,
                        Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                    };
                    let updated_value = match self.declared_variables.get(&variable_name_before) {
                        Some(value) => match value.1.parse::<i32>() {
                            Ok(parsed_value) => parsed_value + increment_value,
                            Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                        },
                        None => return Err(self.error("Variable not found", "Error")),
                    };
                    self.update_variable_value(variable_name_before.clone(), updated_value.to_string())?;
                    self.parse_expression()? // Handle i += 1
                } else {
                    return Err(self.error("Expected '+' or '=' after '+'", "Error"));
                }
            },
            TokenType::Minus => {
                if self.tokens[self.current].token_type == TokenType::Minus {
                    self.current += 1; // Consume the second '+'
                    let updated_value = match self.declared_variables.get(&variable_name_before) {
                        Some(value) => match value.1.parse::<i32>() {
                            Ok(parsed_value) => parsed_value - 1,
                            Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                        },
                        None => return Err(self.error("Variable not found", "Error")),
                    };
                    self.update_variable_value(variable_name_before.clone(), updated_value.to_string())?;
                    ExprNode::IntLiteral(1) // Handle i++
                } else if self.tokens[self.current].token_type == TokenType::Assignment {
                    self.current += 1; // Consume the '='
                    let increment_value = match self.tokens[self.current].lexeme.parse::<i32>() {
                        Ok(value) => value,
                        Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                    };
                    let updated_value = match self.declared_variables.get(&variable_name_before) {
                        Some(value) => match value.1.parse::<i32>() {
                            Ok(parsed_value) => parsed_value - increment_value,
                            Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                        },
                        None => return Err(self.error("Variable not found", "Error")),
                    };
                    self.update_variable_value(variable_name_before.clone(), updated_value.to_string())?;
                    self.parse_expression()? // Handle i += 1
                } else {
                    return Err(self.error("Expected '+' or '=' after '+'", "Error"));
                }
            },
            TokenType::Assignment => { // x = x + 1
                if self.tokens[self.current].token_type == TokenType::Variable
                    && self.is_variable_declared(&self.tokens[self.current].lexeme)
                    && self.tokens[self.current + 1].token_type == TokenType::Plus {
                    let variable_name = self.tokens[self.current].lexeme.clone();
                    self.current += 2; // Consume the variable and '+'
                    if  TokenType::IntegerLiteral == self.tokens[self.current].token_type
                        || TokenType::Variable == self.tokens[self.current].token_type {

                        let increment_value = match self.tokens[self.current].token_type {
                            TokenType::IntegerLiteral => self.tokens[self.current].lexeme.parse::<i32>().unwrap_or_default(),
                            TokenType::Variable => {
                                let variable_name = &self.tokens[self.current].lexeme;
                                match self.declared_variables.get(variable_name) {
                                    Some((_, value)) => value.parse::<i32>().unwrap_or_default(),
                                    None => return Err(self.error("Variable not found", "Error")),
                                }
                            },
                            _ => return Err(self.error("Expected a valid integer or variable", "Error")),
                        };

                        println!("Before {}", self.declared_variables.get(&variable_name).unwrap().1);
                        let variable_value = match self.declared_variables.get(&variable_name) {
                            Some(value) => match value.1.parse::<i32>() {
                                Ok(parsed_value) => parsed_value,
                                Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                            },
                            None => return Err(self.error("Variable not found", "Error")),
                        };
                        let updated_value = variable_value + increment_value;
                        self.update_variable_value(variable_name_before.clone(), updated_value.to_string())?;
                        println!("After {}", self.declared_variables.get(&variable_name).unwrap().1);
                        self.current += 1;
                        ExprNode::IntLiteral(updated_value) // Handle i = i + n
                    } else {
                        return Err(self.error("Expected an integer after '+'", "Error"));
                    }


                }
                else if self.tokens[self.current].token_type == TokenType::Variable
                    && self.is_variable_declared(&self.tokens[self.current].lexeme)
                        && self.tokens[self.current + 1].token_type == TokenType::Minus {
                    let variable_name = self.tokens[self.current].lexeme.clone();
                    self.current += 2; // Consume the variable and '+'
                    if  TokenType::IntegerLiteral == self.tokens[self.current].token_type
                        || TokenType::Variable == self.tokens[self.current].token_type {

                        let increment_value = match self.tokens[self.current].token_type {
                            TokenType::IntegerLiteral => self.tokens[self.current].lexeme.parse::<i32>().unwrap_or_default(),
                            TokenType::Variable => {
                                let variable_name = &self.tokens[self.current].lexeme;
                                match self.declared_variables.get(variable_name) {
                                    Some((_, value)) => value.parse::<i32>().unwrap_or_default(),
                                    None => return Err(self.error("Variable not found", "Error")),
                                }
                            },
                            _ => return Err(self.error("Expected a valid integer or variable", "Error")),
                        };

                        println!("Before {}", self.declared_variables.get(&variable_name).unwrap().1);
                        let variable_value = match self.declared_variables.get(&variable_name) {
                            Some(value) => match value.1.parse::<i32>() {
                                Ok(parsed_value) => parsed_value,
                                Err(_) => return Err(self.error("Expected a valid integer", "Error")),
                            },
                            None => return Err(self.error("Variable not found", "Error")),
                        };
                        let updated_value = variable_value - increment_value;
                        self.update_variable_value(variable_name_before.clone(), updated_value.to_string())?;
                        println!("After {}", self.declared_variables.get(&variable_name).unwrap().1);
                        self.current += 1;
                        ExprNode::IntLiteral(updated_value) // Handle i = i + n
                    } else {
                        return Err(self.error("Expected an integer after '+'", "Error"));
                    }


                } else {
                    self.parse_expression()? // Handle i = expression
                }
            },
            _ => return Err(self.error("Expected an assignment operator", "Error")),
        };


        // println!("current AFTER: {:?}", self.tokens[self.current]);

        let right_type = self.get_expr_type(&right)?;
        if right_type != variable_type && self.is_valid_variable_type(&variable_type){
            match (right_type, variable_type.clone()) {
                (TokenType::Int, _) => return Err(self.error(&format!("Syntax Error: Cannot assign an Integer to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::Float, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a Float to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::Bool, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a Boolean to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::Char, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a Char to a variable of type '{:?}'", variable_type), "Error")),
                (TokenType::String, _) => return Err(self.error(&format!("Syntax Error: Cannot assign a String to a variable of type '{:?}'", variable_type), "Error")),
                _ => (),
            }
        }

        let expr = ExprNode::Binary(Box::new(ExprNode::Variable(variable_name_before.clone())), operator, Box::new(right));
        if self.match_token(TokenType::Semicolon).is_none() {
            self.errors.push(self.error("Expected a semicolon", "Error"));
        }
        self.current -= 1;

        Ok(StmtNode::Assignment(variable_name_before, expr))
    }

    fn parse_expression(&mut self) -> Result<ExprNode, ErrorMessage> {
        let mut expr = self.parse_term()?;

        while let Some(operator) = self.match_token(TokenType::Plus).or(self.match_token(TokenType::Minus)) {
            let right = self.parse_term()?;
            expr = ExprNode::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<ExprNode, ErrorMessage> {
        let mut expr = self.parse_factor()?;

        while let Some(operator) = self.match_token(TokenType::Multiply).or(self.match_token(TokenType::Divide)) {
            let right = self.parse_factor()?;
            expr = ExprNode::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<ExprNode, ErrorMessage> {
        let token = self.tokens[self.current].clone();

        println!("toXXXXXXXXXken: {:?}", token);
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
                        println!("FUCKED UPPPPP %%%%%%%%!!!!!!!!!!");
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
                if !self.is_variable_declared(&token.lexeme) {
                    self.errors.push(self.error(&format!("Use of undeclared variable '{}'", token.lexeme), "Error"));                }
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
                            TokenType::LessThan if l_val < r_val => self.errors.push(self.error("Warning: This condition is always true", "Warning")),
                            TokenType::Equal if l_val == r_val => (),
                            TokenType::NotEqual if l_val != r_val => (),
                            TokenType::GreaterThanOrEqual if l_val >= r_val => (),
                            TokenType::LessThanOrEqual if l_val <= r_val => self.errors.push(self.error("Warning: This condition is always true", "Warning")),
                            _ => self.errors.push(self.error("Warning: This condition is always false", "Warning")),
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
                        if *value {
                            self.errors.push(self.error("Warning: This condition is always true", "Warning"));
                        }
                    },
                    ExprNode::IntLiteral(value) => {
                        if *value != 0 {
                            self.errors.push(self.error("Warning: This condition is always true", "Warning"));
                        } else {
                            self.errors.push(self.error("Warning: This condition is always false", "Warning"));
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
            self.errors.push(self.error("Expected 'if'", "Error"));
        }

        if self.match_token(TokenType::OpenParen).is_none() {
            self.errors.push(self.error("Expected '('", "Error"));
        }

        let condition = match self.parse_condition() {
            Ok(condition) => condition,
            Err(err) if err.message_type == "Warning" => {
                println!("Warning: {:?}", err.message);
                self.errors.push(err);
                ExprNode::BoolLiteral(false) // Use a default value
            },
            Err(err) => return Err(err),
        };

        if self.match_token(TokenType::CloseParen).is_none() {
            self.errors.push(self.error("Expected ')'", "Error"));
        }

        let then_branch = self.parse_block(0)?;
        // self.current += 1;
        // println!("current parse_if_statement: {:?}, {:?}", self.current, self.tokens[self.current]);
        let else_branch = if self.match_token(TokenType::Else).is_some() {
            if self.match_token(TokenType::If).is_some() {
                // Handle 'else if'
                self.current -= 1; // Go back to the 'if' token
                Some(self.parse_if_statement()?)
            } else {
                // Handle 'else'
                Some(self.parse_block(0)?)
            }
        } else {
            self.current -= 1;
            println!("wedwefqwefqwefqwefpk3,ewplf,d1p3l,ef[p1l3,ef13[{:?}ple,f", self.current);
            None
        };

        Ok(StmtNode::IfStatement(condition, Box::new(then_branch), else_branch.map(Box::new)))
    }

    fn parse_declaration_without_semicolon(&mut self) -> Result<StmtNode, ErrorMessage> {
        let variable_type = self.tokens[self.current].token_type.clone();
        self.current += 1; // Consume the type identifier

        let variable_token = self.tokens[self.current].clone();
        if variable_token.token_global != TokenGlobal::Variable {
            self.errors.push(self.error("Expected a variable", "Error"));
        }
        let variable_name = variable_token.lexeme;
        self.declared_variables.insert(variable_name.clone(), (variable_type.clone(), "none".to_string()));
        self.declared_variables.get(&variable_name).cloned().map(|(token_type, _)| token_type).ok_or_else(|| self.error(&format!("Undeclared variable '{}'", variable_name), "Error"));

        let assignment = self.parse_assignment_without_semicolon()?;


        Ok(assignment)
    }

    fn parse_assignment_without_semicolon(&mut self) -> Result<StmtNode, ErrorMessage> {
        let variable_name = self.tokens[self.current].lexeme.clone();

        let variable_type = self.declared_variables.get(&variable_name).cloned().unwrap_or((TokenType::Error, "none".to_string())).0;

        if !self.is_variable_declared(&variable_name) {
            self.errors.push(self.error(&format!("Use of undeclared variable '{}'", variable_name), "Error"));
        }
        self.current += 1; // Consume the variable

        let operator = self.tokens[self.current].token_type.clone();
        self.current += 1; // Consume the operator

        let right = match operator {
            TokenType::Plus => {
                if self.tokens[self.current].token_type == TokenType::Plus {
                    self.current += 1; // Consume the second '+'
                    ExprNode::IntLiteral(1) // Handle i++
                } else if self.tokens[self.current].token_type == TokenType::Assignment {
                    self.current += 1; // Consume the '='
                    self.parse_expression()? // Handle i += 1
                } else {
                    return Err(self.error("Expected '+' or '=' after '+'", "Error"));
                }
            },
            TokenType::Minus => {
                if self.tokens[self.current].token_type == TokenType::Minus {
                    self.current += 1; // Consume the second '+'
                    ExprNode::IntLiteral(-1) // Handle i++
                } else if self.tokens[self.current].token_type == TokenType::Assignment {
                    self.current += 1; // Consume the '='
                    self.parse_expression()? // Handle i += 1
                } else {
                    return Err(self.error("Expected '+' or '=' after '+'", "Error"));
                }
            },
            TokenType::Assignment => {
                if self.tokens[self.current].token_type == TokenType::Variable
                    && self.tokens[self.current].lexeme == variable_name
                    && self.tokens[self.current + 1].token_type == TokenType::Plus {
                    self.current += 3; // Consume the variable, '+', and the number
                    ExprNode::IntLiteral(self.tokens[self.current - 1].lexeme.parse::<i32>().unwrap()) // Handle i = i + 1
                } else {
                    self.parse_expression()? // Handle i = expression
                }
            },
            _ => return Err(self.error("Expected an assignment operator", "Error")),
        };

        let expr = ExprNode::Binary(Box::new(ExprNode::Variable(variable_name.clone())), operator, Box::new(right.clone()));
        self.declared_variables.insert(variable_name.clone(), (variable_type.clone(), right.to_string()));

        Ok(StmtNode::Assignment(variable_name, expr))
    }

    fn parse_single_statement(&mut self) -> Result<StmtNode, ErrorMessage> {
        let stmt = self.parse_statement()?;
        println!("stmt: {:?}", stmt);
        self.current -= 1;

        // println!("current parse_single_statement: {:?}", self.current);
        if self.match_token(TokenType::Semicolon).is_none() {
            println!("Fiiiiirrrrreeeee: {:?}", self.tokens[self.current]);
            self.errors.push(self.error("Expected a semicolon", "Error"));
        }
        Ok(stmt)
    }

    fn parse_for_statement(&mut self) -> Result<StmtNode, ErrorMessage> {
        if self.match_token(TokenType::For).is_none() {
            self.errors.push(self.error("Expected 'for'", "Error"));
        }

        if self.match_token(TokenType::OpenParen).is_none() {
            self.errors.push(self.error("Expected '('", "Error"));
        }

        let initialization = if self.tokens[self.current].token_type != TokenType::Semicolon {
            if self.tokens[self.current].token_global == TokenGlobal::Identifier {
                Some(self.parse_declaration_without_semicolon()?)
            } else {
                Some(self.parse_assignment_without_semicolon()?)
            }
        } else {
            None
        };

        if self.match_token(TokenType::Semicolon).is_none() {
            self.errors.push(self.error("Expected ';'", "Error"));
        }

        let condition = if self.tokens[self.current].token_type != TokenType::Semicolon {
            let condition = self.parse_condition()?;
            match &condition {
                ExprNode::BoolLiteral(value) => {
                    if *value {
                        self.errors.push(self.error("Warning: Endless loop", "Warning"));
                    }
                },
                _ => (),
            }
            Some(condition)
        } else {
            None
        };

        if self.match_token(TokenType::Semicolon).is_none() {
            self.errors.push(self.error("Expected ';'", "Error"));
        }

        let increment = if self.tokens[self.current].token_type != TokenType::CloseParen {
            Some(self.parse_assignment_without_semicolon()?)
        } else {
            None
        };

        if self.match_token(TokenType::CloseParen).is_none() {
            self.errors.push(self.error("Expected ')'", "Error"));
        }

        let statement = self.parse_block(0)?;
        println!("current after FOR LOOP: {:?}", self.current);


        Ok(StmtNode::ForLoop(
            initialization.map(Box::new).unwrap_or(Box::new(StmtNode::Block(vec![]))),
            condition.map(Box::new).unwrap_or(Box::new(ExprNode::BoolLiteral(true))),
            increment.map(Box::new).unwrap_or(Box::new(StmtNode::Block(vec![]))),
            Box::new(statement)
        ))
    }

    fn parse_while_loop(&mut self) -> Result<StmtNode, ErrorMessage> {
        println!("ANA GEEEEEEET");
        if self.match_token(TokenType::While).is_none() {
            return Err(self.error("Expected 'while'", "Error"));
        }

        if self.match_token(TokenType::OpenParen).is_none() {
            return Err(self.error("Expected '('", "Error"));
        }

        let condition = self.parse_condition()?;

        if self.match_token(TokenType::CloseParen).is_none() {
            return Err(self.error("Expected ')'", "Error"));
        }

        let body = self.parse_block(0)?;

        Ok(StmtNode::WhileLoop(Box::new(condition), Box::new(body)))
    }

    fn parse_do_while_loop(&mut self) -> Result<StmtNode, ErrorMessage> {
        if self.match_token(TokenType::Do).is_none() {
            return Err(self.error("Expected 'do'", "Error"));
        }

        let body = self.parse_block(1)?;
        // self.current += 1;
        println!("current parse_do_while_loop: {:?}", self.current);

        if self.match_token(TokenType::While).is_none() {
            return Err(self.error("Expected 'while'", "Error"));
        }

        if self.match_token(TokenType::OpenParen).is_none() {
            return Err(self.error("Expected '('", "Error"));
        }

        let condition = self.parse_condition()?;

        if self.match_token(TokenType::CloseParen).is_none() {
            return Err(self.error("Expected ')'", "Error"));
        }

        if self.match_token(TokenType::Semicolon).is_none() {
            return Err(self.error("Expected ';'", "Error"));
        }

        Ok(StmtNode::DoWhileLoop(Box::new(condition), Box::new(body)))
    }

    fn parse_case_clause(&mut self) -> Result<(ExprNode, StmtNode), ErrorMessage> {
        if self.match_token(TokenType::Case).is_none() {
            return Err(self.error("Expected 'case'", "Error"));
        }
        let case_expr = self.parse_expression()?;
        println!("Expr: {:?}", case_expr);
        if self.match_token(TokenType::Colon).is_none() {
            return Err(self.error("Expected ':'", "Error"));
        }

        let case_stmt = if self.tokens[self.current].token_type == TokenType::For ||  self.tokens[self.current].token_type == TokenType::While || self.tokens[self.current].token_type == TokenType::Do || self.tokens[self.current].token_type == TokenType::If {
            // self.current += 1;
            println!("IAM GOING TO WHULE");
            self.parse_statement()?
        } else if self.tokens[self.current].token_type != TokenType::Break {
            self.parse_single_statement()?
        } else {
            StmtNode::Block(vec![]) // Add an else branch that returns an empty StmtNode
        };
        println!("current parse_case_clause: {:?} {:?}", self.current, self.tokens[self.current]);

        // Check for 'break' statement
        if self.match_token(TokenType::Break).is_none() {
            return Err(self.error("Expected 'break'", "Error"));
        }

        // Check for semicolon after 'break'
        if self.match_token(TokenType::Semicolon).is_none() {
            return Err(self.error("Expected ';'", "Error"));
        }

        Ok((case_expr, case_stmt))
    }

    fn parse_switch_case(&mut self) -> Result<StmtNode, ErrorMessage> {
        if self.match_token(TokenType::Switch).is_none() {
            return Err(self.error("Expected 'switch'", "Error"));
        }


        if self.match_token(TokenType::OpenParen).is_none() {
            return Err(self.error("Expected '('", "Error"));
        }

        let condition = match self.tokens[self.current].token_type {
            TokenType::Variable => self.parse_expression()?,
            _ => return Err(self.error("Expected a variable", "Error")),
        };

        println!("I FOUND SWIITTTTCCCCHHHCHCHCHCHCHHC");
        if self.match_token(TokenType::CloseParen).is_none() {
            return Err(self.error("Expected ')'", "Error"));
        }

        if self.match_token(TokenType::OpenBrace).is_none() {
            return Err(self.error("Expected '{'", "Error"));
        }

        let mut cases = Vec::new();
        while self.tokens[self.current].token_type == TokenType::Case {
            let case_clause = self.parse_case_clause()?;
            cases.push(case_clause);
        }

        println!("cases len: {:?}", cases.len());
        println!("current parse_switch_case: {:?}", self.current);

        if self.match_token(TokenType::CloseBrace).is_none() {
            return Err(self.error("Expected '}'", "Error"));
        }

        Ok(StmtNode::SwitchCase(Box::new(condition), cases))
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

    fn is_valid_variable_type(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::Int | TokenType::Float | TokenType::Bool | TokenType::String | TokenType::Double | TokenType::Char => true,
            _ => false,
        }
    }

    fn get_variable_type(&self, variable_name: &str) -> Result<TokenType, ErrorMessage> {
        self.declared_variables.get(variable_name).cloned().map(|(token_type, _)| token_type).ok_or_else(|| self.error(&format!("use of undeclared variable '{}'", variable_name), "Error"))
    }

    pub fn update_variable_value(&mut self, variable_name: String, new_value: String) -> Result<(), ErrorMessage> {
        if let Some((token_type, _)) = self.declared_variables.get(&variable_name) {
            self.declared_variables.insert(variable_name, (token_type.clone(), new_value));
            Ok(())
        } else {
            Err(self.error(&format!("Undeclared variable '{}'", variable_name), "Error"))
        }
    }

    fn get_expr_type(&self, expr: &ExprNode) -> Result<TokenType, ErrorMessage> {
        match expr {
            ExprNode::IntLiteral(_) => Ok(TokenType::Int),
            ExprNode::FloatLiteral(_) => Ok(TokenType::Float),
            ExprNode::CharLiteral(_) => Ok(TokenType::Char),
            ExprNode::StringLiteral(_) => Ok(TokenType::String),
            ExprNode::Variable(name) => self.get_variable_type(name),
            _ => Err(self.error("Expression is not a literal or a variable", "Error")),
        }
    }

    pub fn get_declared_variables(&self) -> HashMap<String, (TokenType, String)> {
        self.declared_variables.clone()
    }

    fn error(&self, message: &str, message_type_: &str) -> ErrorMessage {
        let token = &self.tokens[if self.current >= self.tokens.len() { self.current - 1 } else { self.current }];
        let error_message = ErrorMessage {
            message_type: message_type_.to_string(),
            message: format!("{}", message),
            line: token.original_line,
            column: token.original_column,

        };
        error_message
    }
}