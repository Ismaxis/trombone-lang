pub type Program = Vec<FuncDeclaration>;

pub type Identifier = String;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FuncDeclaration {
    pub identifier: Identifier,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub statements: Vec<Statement>,
}

impl FuncDeclaration {
    pub fn new(
        identifier: Identifier,
        params: Vec<Param>,
        return_type: Option<Type>,
        statements: Vec<Statement>,
    ) -> Self {
        Self {
            identifier,
            params,
            return_type,
            statements,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Param {
    pub identifier: Identifier,
    pub type_: Type,
}

impl Param {
    pub fn new(identifier: Identifier, type_: Type) -> Self {
        Self { identifier, type_ }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    ArrInt,
}

pub type Block = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    VarDeclaration {
        identifier: Identifier,
        type_: Type,
        value: Expression,
    },
    Assignment {
        identifier: Identifier,
        value: Expression,
    },
    ArrayAssignment {
        identifier: Identifier,
        index: Expression,
        value: Expression,
    },
    ReturnStatement {
        return_value: Option<Expression>,
    },
    WhileStatement {
        condition: Expression,
        statements: Block,
    },
    IfStatement {
        arms: Vec<(Expression, Block)>,
        el: Option<Block>,
    },
    ExpressionStatement {
        expression: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Mul {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Div {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Sub {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Less {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Greater {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    LessEq {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    GreaterEq {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Eq {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    NonEq {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    // Terms
    ArrayAccess {
        identifier: Identifier,
        index: Box<Expression>,
    },
    FuncCall {
        identifier: Identifier,
        arguments: Vec<Expression>,
    },
    UnaryMinus {
        val: Box<Expression>,
    },
    Literal {
        val: i32,
    },
    VarReference {
        identifier: Identifier,
    },
}
