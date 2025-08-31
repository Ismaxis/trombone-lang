pub type Program = Vec<FuncDeclaration>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct FuncDeclaration {
    pub identifier: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub statements: Vec<Statement>,
}

impl FuncDeclaration {
    pub fn new(
        identifier: String,
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
#[derive(Debug)]
pub struct Param {
    pub identifier: String,
    pub type_: Type,
}

impl Param {
    pub fn new(identifier: String, type_: Type) -> Self {
        Self { identifier, type_ }
    }
}

#[derive(Debug)]
pub enum Type {
    Int,
    ArrInt,
}

pub type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    VarDeclaration {
        identifier: String,
        type_: Type,
        value: Expression,
    },
    Assignment {
        identifier: String,
        value: Expression,
    },
    ArrayAssignment {
        identifier: String,
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

#[derive(Debug)]
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
        identifier: String,
        index: Box<Expression>,
    },
    FuncCall {
        identifier: String,
        arguments: Vec<Expression>,
    },
    ArrayCreate {
        arguments: Vec<Expression>,
    },
    UnaryMinus {
        val: Box<Expression>,
    },
    Literal {
        val: i32,
    },
    VarReference {
        identifier: String,
    },
}
