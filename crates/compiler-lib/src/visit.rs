#![allow(dead_code)]

use std::collections::HashMap;

use crate::ast::Expression;

use trombone_common::bytecode::Operation;

use crate::ast::{FuncDeclaration, Identifier, Statement};

#[derive(Debug, Clone)]
pub enum OperationPrototype {
    Defined(Operation),
    Call {
        identifier: Identifier,
    },
    Jump,
    JumpIf,
    JumpIfNot,
    SetRetAddress {
        operands_count: trombone_common::bytecode::Literal,
    },
}

impl OperationPrototype {
    fn calc_stack_diff(&self) -> isize {
        let unused_value = 1337;
        match self {
            Self::Defined(op) => op.calc_stack_diff(),
            Self::Call { identifier: _ } => Operation::Jump {
                offset: unused_value,
            }
            .calc_stack_diff(),
            Self::Jump => Operation::Jump {
                offset: unused_value,
            }
            .calc_stack_diff(),
            Self::JumpIf => Operation::JumpIf {
                offset: unused_value,
            }
            .calc_stack_diff(),
            Self::JumpIfNot => Operation::JumpIfNot {
                offset: unused_value,
            }
            .calc_stack_diff(),
            Self::SetRetAddress { operands_count } => Operation::SetRetAddress {
                operands_count: *operands_count,
            }
            .calc_stack_diff(),
        }
    }
}

impl From<Operation> for OperationPrototype {
    fn from(value: Operation) -> Self {
        match value {
            Operation::SetRetAddress { operands_count } => Self::SetRetAddress {
                operands_count: operands_count,
            },
            Operation::Jump { offset: _ } => Self::Jump,
            Operation::JumpIf { offset: _ } => Self::JumpIf,
            Operation::JumpIfNot { offset: _ } => Self::JumpIfNot,
            _ => Self::Defined(value),
        }
    }
}

fn add_operation(ctx: &mut Context, ops: &mut Vec<OperationPrototype>, op: OperationPrototype) {
    let stack_diff = op.calc_stack_diff();
    ops.push(op.clone().into());
    ctx.current_rsp = (ctx.current_rsp as isize + stack_diff) as usize;
}

pub struct Context {
    current_rsp: usize,
    declared_funcs: HashMap<Identifier, FuncMeta>,
    scopes: Vec<Scope>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            current_rsp: 0,
            declared_funcs: HashMap::new(),
            scopes: Vec::new(),
        }
    }

    pub fn declare_func(&mut self, id: &Identifier, meta: FuncMeta) {
        let prev = self.declared_funcs.insert(id.clone(), meta);
        if prev.is_some() {
            panic!("func '{}' already defined", id);
        }
    }

    fn get_func(&self, id: &Identifier) -> Option<FuncMeta> {
        self.declared_funcs.get(id).cloned()
    }

    fn get_var(&mut self, id: &Identifier) -> Option<VarMeta> {
        self.scopes
            .iter()
            .rev()
            .filter_map(|x| x.declared_vars.get(id))
            .next()
            .cloned()
    }

    fn enter_scope(&mut self, tag: ScopeTag) {
        self.scopes.push(Scope {
            start_rsp: self.current_rsp,
            declared_vars: HashMap::new(),
            tag,
        });
    }

    /*
    Next two methods have different semantics. To show difference better, let's
    explore next scenarios, where these methods are used
       Examples:
        - Exit the scope:
            ```trombone
                if ... {
                    let var1: int = 42;
                    let var2: int = 54;
                    ...
                    // <-- you are here
                }
            ```
            In this scenario,
            - calling `destruct_scope_vars` follows pushing two
            Operation::Pop - it pops `var2` and `var1`
            - calling `exit_scope` follows forgetting all information
            about `var1` and `var2`
        - Return from function
            ```trombone
                fn test(n: int) -> int {
                    let var1: int = 42;
                    if n <= 1 {
                        let var2: int = 52;
                        if true {
                            let var3: int = 62;
                            return 12321; // <---- you are here;
                        }
                    }
                    ...
                }
            ```
            - calling `destruct_scope_vars` follows pushing one Operation::Pop - it
            pops var3.
            - calling `exit_scope` follows forgetting `var3`
            In this scenario, you should call destruct_scope_vars for every scope
            until function scope,
            but call exit_scope only for the most inner one.
            Motivation is next: runtime should destroy `var3`, `var2`, `var1` and
            `n`, but compiler should forget only `var3`.

     */

    // Generates operations to destroy variables in current scope
    fn destruct_scope_vars(&mut self) -> Vec<OperationPrototype> {
        let mut vs = self.cur_scope().declared_vars.iter().collect::<Vec<_>>();
        vs.sort_by(|(_, meta1), (_, meta2)| meta1.address.cmp(&meta2.address).reverse());

        vs.iter()
            .map(|x| {
                match x.1.type_ {
                    crate::ast::Type::Int => Operation::Pop,
                    crate::ast::Type::ArrInt => Operation::HeapPopPtr,
                }
                .into()
            })
            .collect::<Vec<_>>()
    }

    fn destruct_all_vars(&mut self) -> Vec<OperationPrototype> {
        let mut vs = self
            .scopes
            .iter()
            .flat_map(|x| x.declared_vars.clone())
            .collect::<Vec<_>>();
        vs.sort_by(|(_, meta1), (_, meta2)| meta1.address.cmp(&meta2.address).reverse());

        vs.iter()
            .map(|x| {
                match x.1.type_ {
                    crate::ast::Type::Int => Operation::Pop,
                    crate::ast::Type::ArrInt => Operation::HeapPopPtr,
                }
                .into()
            })
            .collect::<Vec<_>>()
    }

    // This method make context forget variables in scope
    fn exit_scope(&mut self) {
        let last_scope = self.scopes.pop().unwrap();
        let prev_sp = last_scope.start_rsp;
        debug_assert_eq!(
            prev_sp + last_scope.declared_vars.len(),
            self.current_rsp,
            "Expected only variables on stack, but found temporaries? Expected stack pointer = {}, found = {}",
            prev_sp + last_scope.declared_vars.len(),
            self.current_rsp
        );
        self.current_rsp = prev_sp;
    }

    fn cur_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}

struct Scope {
    start_rsp: usize,
    declared_vars: HashMap<Identifier, VarMeta>,

    tag: ScopeTag,
}

impl Scope {
    fn declare_var(&mut self, id: Identifier, meta: VarMeta) {
        let res = self.declared_vars.insert(id, meta);
        debug_assert!(res.is_none());
    }
}

enum ScopeTag {
    Func,
    Block,
}

// Meta

#[derive(Clone)]
pub struct FuncMeta {
    pub arguments_types: Vec<crate::ast::Type>,
    pub return_type: Option<crate::ast::Type>,
}

#[derive(Clone)]
struct VarMeta {
    address: usize, // rsp at the moment of declaration
    type_: crate::ast::Type,
}

// Impl

impl FuncDeclaration {
    pub fn visit(&self, ctx: &mut Context) -> Vec<OperationPrototype> {
        debug_assert_eq!(
            ctx.current_rsp, 0,
            "should be no stack at the beggining of the func"
        );

        // prep
        ctx.declare_func(
            &self.identifier,
            FuncMeta {
                arguments_types: self.params.iter().map(|x| x.type_).collect(),
                return_type: self.return_type,
            },
        );
        ctx.enter_scope(ScopeTag::Func); // scope for params

        // params
        for param in &self.params {
            let meta = VarMeta {
                address: ctx.current_rsp,
                type_: param.type_,
            };
            ctx.current_rsp += 1;
            ctx.cur_scope().declare_var(param.identifier.clone(), meta)
        }

        // statments
        let ops = self
            .statements
            .iter()
            .map(|x| x.visit(ctx))
            .flatten()
            .collect::<Vec<_>>();

        // operands are cleared in return // ReturnStatement is mandatory
        debug_assert!(
            matches!(
                ops.last(),
                Some(OperationPrototype::Defined(Operation::Return))
            ),
            "return statement is mandatory at the end of func"
        );

        return ops;
    }
}

impl Statement {
    pub fn visit(&self, ctx: &mut Context) -> Vec<OperationPrototype> {
        match self {
            Statement::VarDeclaration {
                identifier,
                type_,
                value,
            } => {
                let address = ctx.current_rsp;
                let ops = value.visit(ctx);
                assert_eq!(
                    address,
                    ctx.current_rsp - 1,
                    "expression evaluation should advance rsp"
                );

                ctx.cur_scope().declare_var(
                    identifier.clone(),
                    VarMeta {
                        address,
                        type_: *type_,
                    },
                );

                ops
            }
            Statement::Assignment { identifier, value } => {
                if let Some(var_meta) = ctx.get_var(identifier) {
                    let mut ops = value.visit(ctx);
                    match var_meta.type_ {
                        crate::ast::Type::Int => {
                            let offset = ctx.current_rsp - 1 - var_meta.address - 1; // TODO: check
                            let op = Operation::LocalStore {
                                variable_offset: offset as i32,
                            };
                            add_operation(ctx, &mut ops, op.into());
                            return ops;
                        }
                        crate::ast::Type::ArrInt => todo!("Not implemented assignment to array"),
                    }
                } else {
                    todo!("variable not found (implement error handling)")
                }
            }
            Statement::ArrayAssignment {
                identifier: _,
                index: _,
                value: _,
            } => todo!("ArrayAssignment"),
            Statement::ReturnStatement { return_value } => {
                let mut ops = Vec::new();
                if let Some(return_value) = return_value {
                    ops = return_value.visit(ctx);
                    let return_value_offset = (ctx.current_rsp - 1) as i32; // TODO: check; abstract calculating it in other method
                    let set_return_value_op = Operation::LocalStore {
                        variable_offset: return_value_offset,
                    };
                    add_operation(ctx, &mut ops, set_return_value_op.into());
                }
                ops.append(&mut ctx.destruct_all_vars());
                add_operation(ctx, &mut ops, Operation::Return.into());
                ops
            }
            Statement::WhileStatement {
                condition: _,
                statements: _,
            } => todo!("WhileStatement"),
            Statement::IfStatement { arms: _, el: _ } => todo!("IfStatement"),
            Statement::ExpressionStatement { expression } => {
                let mut ops = expression.visit(ctx);
                if let Some(expr_type) = expression.get_type(ctx) {
                    discard_value(expr_type, &mut ops);
                }
                ops
            }
        }
    }
}

fn discard_value(return_type: crate::ast::Type, ops: &mut Vec<OperationPrototype>) {
    let pop_op = match return_type {
        crate::ast::Type::Int => Operation::Pop.into(),
        crate::ast::Type::ArrInt => Operation::HeapPopPtr.into(),
    };
    ops.push(pop_op);
}

impl Expression {
    pub fn visit(&self, ctx: &mut Context) -> Vec<OperationPrototype> {
        match self {
            Expression::Mul { lhs, rhs }
            | Expression::Div { lhs, rhs }
            | Expression::Add { lhs, rhs }
            | Expression::Sub { lhs, rhs }
            | Expression::Less { lhs, rhs }
            | Expression::Greater { lhs, rhs }
            | Expression::LessEq { lhs, rhs }
            | Expression::GreaterEq { lhs, rhs }
            | Expression::Eq { lhs, rhs }
            | Expression::NonEq { lhs, rhs } => {
                let mut ops = Vec::new();
                ops.extend(lhs.visit(ctx));
                ops.extend(rhs.visit(ctx));

                add_operation(ctx, &mut ops, self.get_operation().into());

                ops
            }
            Expression::ArrayAccess {
                identifier: _,
                index: _,
            } => todo!("Expression::ArrayAccess"),
            Expression::FuncCall {
                identifier,
                arguments,
            } => {
                if let Some(fn_meta) = ctx.get_func(identifier) {
                    // check types
                    let given_types = arguments
                        .iter()
                        .map(|x| x.get_type(ctx))
                        .collect::<Vec<_>>();
                    let declared_types = fn_meta
                        .arguments_types
                        .iter()
                        .cloned()
                        .map(|x| Some(x))
                        .collect::<Vec<_>>();
                    if given_types != declared_types {
                        // TODO: compare iterators and eval only on error
                        panic!(
                            "given types does not match declared: {:?} != {:?}",
                            given_types, declared_types
                        )
                    }

                    let mut ops = Vec::new();
                    if handle_builtins(ctx, &mut ops, identifier, arguments) {
                        return ops;
                    }

                    let arg_count = arguments.len();
                    if fn_meta.return_type.is_some() {
                        reserve(ctx, &mut ops); // for return value
                    }
                    reserve(ctx, &mut ops); // for return address

                    eval_arguments(ctx, &mut ops, arguments);

                    add_operation(
                        ctx,
                        &mut ops,
                        Operation::SetRetAddress {
                            operands_count: arg_count as i32,
                        }
                        .into(),
                    );
                    // call
                    add_operation(
                        ctx,
                        &mut ops,
                        OperationPrototype::Call {
                            identifier: identifier.clone(),
                        },
                    );
                    ops
                } else {
                    panic!("func '{}' not found", identifier);
                }
            }
            Expression::UnaryMinus { val: _ } => todo!("Expression::UnaryMinus"),
            Expression::Literal { val } => {
                let mut ops = Vec::new();
                add_operation(ctx, &mut ops, Operation::PushLiteral { value: *val }.into());
                ops
            }
            Expression::VarReference { identifier } => {
                let mut ops = Vec::new();

                if let Some(var_meta) = ctx.get_var(identifier) {
                    add_operation(
                        ctx,
                        &mut ops,
                        Operation::LocalCopy {
                            variable_offset: ctx.current_rsp as i32 - 1 - var_meta.address as i32,
                        }
                        .into(),
                    );
                } else {
                    panic!("var '{}' not found", identifier);
                }

                ops
            }
        }
    }

    fn get_type(&self, ctx: &mut Context) -> Option<crate::ast::Type> {
        match self {
            Expression::Mul { lhs: _, rhs: _ }
            | Expression::Div { lhs: _, rhs: _ }
            | Expression::Add { lhs: _, rhs: _ }
            | Expression::Sub { lhs: _, rhs: _ }
            | Expression::Less { lhs: _, rhs: _ }
            | Expression::Greater { lhs: _, rhs: _ }
            | Expression::LessEq { lhs: _, rhs: _ }
            | Expression::GreaterEq { lhs: _, rhs: _ }
            | Expression::Eq { lhs: _, rhs: _ }
            | Expression::NonEq { lhs: _, rhs: _ } => Some(crate::ast::Type::Int),
            Expression::ArrayAccess {
                identifier: _,
                index: _,
            } => {
                // NOTE: for now only ints can be stored in array
                Some(crate::ast::Type::Int)
            }
            Expression::FuncCall {
                identifier,
                arguments: _,
            } => {
                if let Some(fn_meta) = ctx.get_func(&identifier) {
                    fn_meta.return_type.clone()
                } else {
                    panic!("func '{}' not found", identifier);
                }
            }
            Expression::UnaryMinus { val: _ } => Some(crate::ast::Type::Int),
            Expression::Literal { val: _ } => Some(crate::ast::Type::Int),
            Expression::VarReference { identifier } => {
                if let Some(var_meta) = ctx.get_var(identifier) {
                    Some(var_meta.type_)
                } else {
                    panic!("var '{}' not found", identifier);
                }
            }
        }
    }

    fn get_operation(&self) -> Operation {
        match self {
            Expression::Mul { lhs: _, rhs: _ } => Operation::Mul,
            Expression::Div { lhs: _, rhs: _ } => Operation::Div,
            Expression::Add { lhs: _, rhs: _ } => Operation::Add,
            Expression::Sub { lhs: _, rhs: _ } => Operation::Sub,
            Expression::Less { lhs: _, rhs: _ } => Operation::LessThan,
            Expression::Greater { lhs: _, rhs: _ } => Operation::GreaterThan,
            Expression::LessEq { lhs: _, rhs: _ } => Operation::LessThanOrEqual,
            Expression::GreaterEq { lhs: _, rhs: _ } => Operation::GreaterThanOrEqual,
            Expression::Eq { lhs: _, rhs: _ } => Operation::Equal,
            Expression::NonEq { lhs: _, rhs: _ } => Operation::NotEqual,
            Expression::UnaryMinus { val: _ } => Operation::Not,
            other => panic!("no operaton for '{:?}'", other),
        }
    }
}

fn eval_arguments(
    ctx: &mut Context,
    ops: &mut Vec<OperationPrototype>,
    arguments: &Vec<Expression>,
) {
    ops.extend(arguments.iter().map(|x| x.visit(ctx)).flatten());
}

fn handle_builtins(
    ctx: &mut Context,
    ops: &mut Vec<OperationPrototype>,
    identifier: &Identifier,
    arguments: &Vec<Expression>,
) -> bool {
    match identifier.as_str() {
        "print" => {
            eval_arguments(ctx, ops, arguments);
            add_operation(ctx, ops, Operation::Print.into());
            true
        }
        "read" => {
            eval_arguments(ctx, ops, arguments);
            add_operation(ctx, ops, Operation::Read.into());
            true
        }
        "array" => todo!("builtin: array(n)"),
        _ => false,
    }
}

fn reserve(ctx: &mut Context, ops: &mut Vec<OperationPrototype>) {
    let unused_value = 1338;
    add_operation(
        ctx,
        ops,
        Operation::PushLiteral {
            value: unused_value,
        }
        .into(),
    );
}
