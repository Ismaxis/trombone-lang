use std::collections::HashMap;

use trombone_common::{
    bytecode::{Instruction, OperationStream},
    opcode,
};

pub enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Let,
}

pub enum Param {
    Val { value: i64 },
    Var { value: String },
    Node(Box<MinimalAstNode>),
}

pub struct MinimalAstNode {
    pub op: Operation,
    pub params: [Param; 2],
}

enum Token {
    OpenParen,
    CloseParen,
    Ident(String),
    Number(i64),
}

pub fn compile(s: &str) -> OperationStream<'_> {
    let ast = make_ast_tree(s);

    let mut instructions = [0u64; 1024];
    OperationStream::new(&instructions);
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            c if c.is_whitespace() => continue,
            c if c.is_ascii_digit() => {
                let mut num = String::new();
                num.push(c);
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n = num.parse().expect("Invalid number");
                tokens.push(Token::Number(n));
            }
            c if c.is_ascii_alphabetic() => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_alphanumeric() {
                        ident.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(ident));
            }
            _ => panic!("Invalid character: {}", c),
        }
    }
    tokens
}

fn parse_param(tokens: &[Token], pos: &mut usize) -> Param {
    match tokens.get(*pos) {
        Some(Token::OpenParen) => {
            *pos += 1;
            let node = parse_node(tokens, pos);
            if !matches!(tokens.get(*pos), Some(Token::CloseParen)) {
                panic!("Expected closing parenthesis after node");
            }
            *pos += 1;
            Param::Node(Box::new(node))
        }
        Some(Token::Ident(ident)) => {
            if ident == "Val" {
                *pos += 1;
                if let Some(Token::Number(n)) = tokens.get(*pos) {
                    *pos += 1;
                    Param::Val { value: *n }
                } else {
                    panic!("Expected number after Val");
                }
            } else if ident == "Var" {
                *pos += 1;
                if let Some(Token::Ident(var_name)) = tokens.get(*pos) {
                    *pos += 1;
                    Param::Var {
                        value: var_name.clone(),
                    }
                } else {
                    panic!("Expected identifier after Var");
                }
            } else {
                panic!("Unexpected identifier: {}", ident);
            }
        }
        _ => panic!("Unexpected token in parameter"),
    }
}

fn parse_node(tokens: &[Token], pos: &mut usize) -> MinimalAstNode {
    let op_ident = match tokens.get(*pos) {
        Some(Token::Ident(s)) => s,
        _ => panic!("Expected operator identifier"),
    };
    let op = match op_ident.as_str() {
        "Add" => Operation::Add,
        "Sub" => Operation::Sub,
        "Mult" => Operation::Mult,
        "Div" => Operation::Div,
        "Let" => Operation::Let,
        _ => panic!("Unknown operation: {}", op_ident),
    };
    *pos += 1;
    let param1 = parse_param(tokens, pos);
    let param2 = parse_param(tokens, pos);
    if !matches!(tokens.get(*pos), Some(Token::CloseParen)) {
        panic!("Expected closing parenthesis after node parameters");
    }
    *pos += 1;
    MinimalAstNode {
        op,
        params: [param1, param2],
    }
}

pub fn make_ast_tree(s: &str) -> Vec<MinimalAstNode> {
    s.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let tokens = tokenize(line);
            let mut pos = 0;
            if tokens.is_empty() || !matches!(tokens[0], Token::OpenParen) {
                panic!("AST line must start with an open parenthesis: {}", line);
            }
            pos += 1;
            let node = parse_node(&tokens, &mut pos);
            if pos != tokens.len() {
                panic!("Unexpected tokens at the end of AST line: {}", line);
            }
            Some(node)
        })
        .collect()
}

// Visitor trait defines methods for visiting nodes and parameters
pub trait Visitor {
    type Output;

    // Methods for operations
    fn visit_add(&mut self, left: &Param, right: &Param) -> Self::Output;
    fn visit_sub(&mut self, left: &Param, right: &Param) -> Self::Output;
    fn visit_mult(&mut self, left: &Param, right: &Param) -> Self::Output;
    fn visit_div(&mut self, left: &Param, right: &Param) -> Self::Output;
    fn visit_let(&mut self, var: &Param, expr: &Param) -> Self::Output;

    // Methods for parameters
    fn visit_val(&mut self, value: u64) -> Self::Output;
    fn visit_var(&mut self, name: &str) -> Self::Output;
    fn visit_node(&mut self, node: &MinimalAstNode) -> Self::Output;
}

// Implement `accept` for MinimalAstNode to delegate to the visitor
impl MinimalAstNode {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        match &self.op {
            Operation::Add => visitor.visit_add(&self.params[0], &self.params[1]),
            Operation::Sub => visitor.visit_sub(&self.params[0], &self.params[1]),
            Operation::Mult => visitor.visit_mult(&self.params[0], &self.params[1]),
            Operation::Div => visitor.visit_div(&self.params[0], &self.params[1]),
            Operation::Let => visitor.visit_let(&self.params[0], &self.params[1]),
        }
    }
}

// Implement `accept` for Param to delegate to the visitor
impl Param {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Param::Val { value } => visitor.visit_val(*value),
            Param::Var { value } => visitor.visit_var(value),
            Param::Node(node) => visitor.visit_node(node),
        }
    }
}

// Example visitor: Evaluator computes the result of the AST
pub struct BytecodeGenerator<'a> {
    stream: OperationStream<'a>,
    env: HashMap<String, i64>,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(stream: OperationStream<'a>) -> Self {
        Self {
            stream,
            env: HashMap::new(),
        }
    }

    fn emit(&mut self, value: u64) {
        self.stream.emit(value);
    }
}

impl<'a> Visitor for BytecodeGenerator<'a> {
    type Output = ();

    fn visit_add(&mut self, left: &Param, right: &Param) -> Self::Output {
        left.accept(self);
        right.accept(self);
    }

    fn visit_sub(&mut self, left: &Param, right: &Param) -> Self::Output {
        let l = left.accept(self);
        let r = right.accept(self);
        l - r
    }

    fn visit_mult(&mut self, left: &Param, right: &Param) -> Self::Output {
        let l = left.accept(self);
        let r = right.accept(self);
        l * r
    }

    fn visit_div(&mut self, left: &Param, right: &Param) -> Self::Output {
        let l = left.accept(self);
        let r = right.accept(self);
        l / r
    }

    fn visit_let(&mut self, var: &Param, expr: &Param) -> Self::Output {
        if let Param::Var { value: var_name } = var {
            let val = expr.accept(self);
            self.env.insert(var_name.clone(), val);
            val
        } else {
            panic!("Let expected a variable as first parameter");
        }
    }

    fn visit_val(&mut self, value: i32) -> Self::Output {
        self.emit(Instruction::from_parts(opcode::OP_PUSH, value).as_u64())
    }

    fn visit_var(&mut self, name: &str) -> Self::Output {
        let var_addr = *self
            .env
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name));
        self.emit(Instruction::from_parts(opcode::OP_LOCAL_COPY, var_addr).as_u64());
        // Put var value on stack
    }

    fn visit_node(&mut self, node: &MinimalAstNode) -> Self::Output {
        node.accept(self)
    }
}
