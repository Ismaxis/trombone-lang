#![allow(dead_code)]

use std::{collections::HashMap, iter::once};

use crate::ast::Expression;

use trombone_common::bytecode::Operation;

use crate::ast::{FuncDeclaration, Identifier, Statement};

#[derive(Debug, Clone)]
pub enum OperationPrototype {
    Defined(Operation),
    Call {
        identifier: Identifier,
        arg_count: isize,
    },
    BasicBlockStart,
}

impl OperationPrototype {
    fn calc_stack_diff(&self) -> isize {
        let unused_value = 1337;
        match self {
            Self::Defined(Operation::Return) => 0, // we can return multiple times, return address cleared by top scope return
            Self::Defined(op) => op.calc_stack_diff(),
            Self::Call {
                identifier: _,
                arg_count,
            } => {
                -arg_count - 1
                    + Operation::Jump {
                        offset: unused_value,
                    }
                    .calc_stack_diff()
            }
            Self::BasicBlockStart => todo!("Self::BasicBlockStart"),
        }
    }
}

impl From<Operation> for OperationPrototype {
    fn from(value: Operation) -> Self {
        Self::Defined(value)
    }
}

fn add_operation(ctx: &mut Context, ops: &mut Vec<OperationPrototype>, op: OperationPrototype) {
    let stack_diff = op.calc_stack_diff();
    ops.push(op.clone());
    ctx.current_rsp = (ctx.current_rsp as isize + stack_diff) as usize;
}

pub struct Context {
    current_rsp: usize,
    declared_funcs: HashMap<Identifier, FuncMeta>,
    scopes: Vec<Scope>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            current_rsp: 0,
            declared_funcs: HashMap::new(),
            scopes: Vec::new(),
        }
    }

    pub(crate) fn declare_func(&mut self, id: &Identifier, meta: FuncMeta) {
        let prev = self.declared_funcs.insert(id.clone(), meta);
        if prev.is_some() {
            panic!("func '{}' already defined", id);
        }
    }

    pub(crate) fn get_func(&self, id: &Identifier) -> Option<FuncMeta> {
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
        let mut vs = self.cur_scope().declared_vars.drain().collect::<Vec<_>>();
        vs.sort_by(|(_, meta1), (_, meta2)| meta1.address.cmp(&meta2.address).reverse());

        let ops = vs
            .iter()
            .map(|x| {
                match x.1.type_ {
                    crate::ast::Type::Int => Operation::Pop,
                    crate::ast::Type::ArrInt => Operation::HeapPopPtr,
                }
                .into()
            })
            .collect::<Vec<_>>();

        self.current_rsp = (self.current_rsp as isize
            + ops
                .iter()
                .map(OperationPrototype::calc_stack_diff)
                .sum::<isize>()) as usize;

        ops
    }

    fn destruct_all_vars(&mut self) -> Vec<OperationPrototype> {
        let mut vs = self
            .scopes
            .iter()
            .flat_map(|x| x.declared_vars.clone())
            .collect::<Vec<_>>();
        vs.sort_by(|(_, meta1), (_, meta2)| meta1.address.cmp(&meta2.address).reverse());

        let ops = vs
            .iter()
            .map(|x| {
                match x.1.type_ {
                    crate::ast::Type::Int => Operation::Pop,
                    crate::ast::Type::ArrInt => Operation::HeapPopPtr,
                }
                .into()
            })
            .collect::<Vec<_>>();

        self.current_rsp = (self.current_rsp as isize
            + ops
                .iter()
                .map(OperationPrototype::calc_stack_diff)
                .sum::<isize>()) as usize;
        ops
    }

    // This method make context forget variables in scope
    fn exit_scope(&mut self) {
        let last_scope = self.scopes.pop().unwrap();
        let prev_sp = last_scope.start_rsp;
        // TODO: Revise the assert, taking into account "return value" and "return address" in FuncDeclaration::visit()
        // debug_assert_eq!(
        //     prev_sp + last_scope.declared_vars.len(),
        //     self.current_rsp,
        //     "Expected only variables on stack, but found temporaries? Expected stack pointer = {}, found = {}",
        //     prev_sp + last_scope.declared_vars.len(),
        //     self.current_rsp
        // );
        self.current_rsp = prev_sp;
    }

    fn cur_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
enum ScopeTag {
    FuncParams,
    Block,
}

// Meta

#[derive(Clone)]
pub struct FuncMeta {
    pub arguments_types: Vec<crate::ast::Type>,
    pub return_type: Option<crate::ast::Type>,
}

#[derive(Clone, Debug)]
struct VarMeta {
    address: usize, // rsp at the moment of declaration
    type_: crate::ast::Type,
}

// Impl

impl FuncDeclaration {
    pub fn visit(&self, ctx: &mut Context) -> Vec<OperationPrototype> {
        debug_assert_eq!(
            ctx.current_rsp, 0,
            "should be no stack at the beggining of the func '{}'",
            self.identifier
        );

        // prep
        if self.return_type.is_some() {
            ctx.current_rsp += 1; // reserve for return value
        }
        ctx.current_rsp += 1; // reserve for return address

        ctx.enter_scope(ScopeTag::FuncParams); // scope for params

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
            .flat_map(|x| x.visit(ctx))
            .collect::<Vec<_>>();

        // operands are cleared in return // ReturnStatement is mandatory
        debug_assert!(
            matches!(
                ops.last(),
                Some(OperationPrototype::Defined(Operation::Return))
            ),
            "return statement is mandatory at the end of func"
        );

        ctx.exit_scope();

        if self.return_type.is_some() {
            ctx.current_rsp -= 1; // was reserved for return value
        }

        ctx.current_rsp -= 1; // was reserved for return address

        ops
    }
}

fn visit_statements(ctx: &mut Context, statements: &Vec<Statement>) -> Vec<OperationPrototype> {
    ctx.enter_scope(ScopeTag::Block);
    let mut ops = statements
        .iter()
        .flat_map(|x| x.visit(ctx))
        .collect::<Vec<_>>();

    // important to remove all deadcode statements after first return
    if let Some(Statement::ReturnStatement { .. }) = statements.last() {
        // do not destruct vars, because return already did it
    } else {
        ops.append(&mut ctx.destruct_scope_vars());
    }
    ctx.exit_scope();
    ops
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
                            ops
                        }
                        crate::ast::Type::ArrInt => todo!("Not implemented assignment to array"),
                    }
                } else {
                    panic!("Assignment: var not found {}", identifier);
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
                    let return_value_offset = (ctx.current_rsp - 1 - 1) as i32; // TODO: abstract calculating it in other method
                    let set_return_value_op = Operation::LocalStore {
                        variable_offset: return_value_offset,
                    };
                    add_operation(ctx, &mut ops, set_return_value_op.into());
                }
                // Warning: destruct all vars here make other statements broken,
                // because after destruction all variables, stack becomes zero
                // -> all accesses to variables lead to "attempt to subtract with overflow"

                // TODO: fix it or build on this mechanism 'dead code detection'
                ops.append(&mut ctx.destruct_all_vars());
                add_operation(ctx, &mut ops, Operation::Return.into());
                ops
            }
            Statement::WhileStatement {
                condition,
                statements,
            } => {
                // Ordering of compiling statements_ops before condition_ops is important!
                // We assume that after [execution condition operations and testing it (execution of OP_JMP_IFNOT)], stack pointer will be the same as before execution (check assert)

                let rsp_before = ctx.current_rsp;
                ctx.enter_scope(ScopeTag::Block);

                let mut statements_ops = statements
                    .iter()
                    .flat_map(|x| x.visit(ctx))
                    .collect::<Vec<_>>();

                statements_ops.append(&mut ctx.destruct_scope_vars());
                ctx.exit_scope();
                assert_eq!(
                    ctx.current_rsp, rsp_before,
                    "Statement block evaluation should return stack pointer to previous state"
                );

                let rsp_before = ctx.current_rsp;
                let condition_ops = condition.visit(ctx);
                assert_eq!(
                    ctx.current_rsp as isize
                        + Operation::JumpIfNot { offset: 0xBEEF }.calc_stack_diff(),
                    rsp_before as isize,
                    "Evaluation of condition expression should not affect stack pointer"
                );

                let condition_ops_len = condition_ops.len() as i32;
                let statements_ops_len = statements_ops.len() as i32;

                let mut ops = condition_ops;
                add_operation(
                    ctx,
                    &mut ops,
                    Operation::JumpIfNot {
                        offset: statements_ops_len + 2,
                    }
                    .into(),
                );
                ops.append(&mut statements_ops);
                add_operation(
                    ctx,
                    &mut ops,
                    Operation::Jump {
                        offset: -(statements_ops_len + 1 + condition_ops_len),
                    }
                    .into(),
                );

                ops
            }
            Statement::IfStatement { arms, el } => {
                // TODO: optimization if only 1 if (no else)

                const COND_VAR_SIZE: usize = 1;
                const JUMP_IN_SIZE: usize = 1;
                const JUMP_OUT_SIZE: usize = 1;

                let beg_rsp = ctx.current_rsp;

                let mut ops: Vec<OperationPrototype> = Vec::new();

                let arms_ops: Vec<Vec<OperationPrototype>> = arms
                    .iter()
                    .map(|(_, arm)| {
                        let cur_arm_ops = visit_statements(ctx, arm);
                        debug_assert_eq!(ctx.current_rsp, beg_rsp, "arm didn't affect rsp");
                        cur_arm_ops
                    })
                    .collect();

                let else_ops = el
                    .as_ref()
                    .map_or(vec![], |else_arm| visit_statements(ctx, else_arm));
                debug_assert_eq!(ctx.current_rsp, beg_rsp, "else didn't affect rsp");

                let conds_ops = arms
                    .iter()
                    .map(|(cond, _)| {
                        let ops = cond.visit(ctx);
                        debug_assert_eq!(
                            ctx.current_rsp,
                            beg_rsp + COND_VAR_SIZE,
                            "cond only pushed 1 value"
                        );
                        ctx.current_rsp = beg_rsp; // reset rsp after condition evaluation
                        ops
                    })
                    .collect::<Vec<_>>();

                // pre-calculate jumps offsets
                let arms_cumsum = calculate_cumsum(arms_ops.iter());
                let mut arms_cumsum_rev =
                    calculate_cumsum(arms_ops.iter().chain(once(&else_ops)).rev());
                arms_cumsum_rev.reverse();
                let mut conds_cumsum_rev = calculate_cumsum(conds_ops.iter().rev());
                conds_cumsum_rev.reverse();

                for i in 0..arms.len() {
                    let cond_ops = &conds_ops[i];

                    // condition
                    ops.extend((*cond_ops).iter().cloned());
                    ctx.current_rsp = beg_rsp + COND_VAR_SIZE;

                    // jump in
                    add_operation(
                        ctx,
                        &mut ops,
                        Operation::JumpIf {
                            offset: (1 + // TODO: Because Jump does -1 (for more details, check Runner::evaluate_next_instruction() Jump arm)
                                conds_cumsum_rev[i + 1]
                                + arms_cumsum[i]
                                + JUMP_IN_SIZE * (arms.len() - i)
                                + JUMP_OUT_SIZE * i) as i32,
                        }
                        .into(),
                    );
                    debug_assert_eq!(ctx.current_rsp, beg_rsp, "rsp unchanged after condition");
                }

                let beg_rsp = ctx.current_rsp;
                // jump else
                add_operation(
                    ctx,
                    &mut ops,
                    Operation::Jump {
                        offset: (1 + // TODO: Because Jump does -1 (for more details, check Runner::evaluate_next_instruction() Jump arm)
                            arms_cumsum.last().unwrap() + JUMP_OUT_SIZE * arms.len())
                            as i32,
                    }
                    .into(),
                );
                debug_assert_eq!(ctx.current_rsp, beg_rsp, "rsp unchanged after jumping else");

                for i in 0..arms.len() {
                    let arm_ops = &arms_ops[i];

                    // block
                    ops.extend((*arm_ops).iter().cloned());

                    let beg_rsp = ctx.current_rsp;

                    // jump out
                    add_operation(
                        ctx,
                        &mut ops,
                        Operation::Jump {
                            offset: (1 + // TODO: Because Jump does -1 (for more details, check Runner::evaluate_next_instruction() Jump arm)
                                arms_cumsum_rev[i + 1] + JUMP_OUT_SIZE * (arms.len() - (i + 1)))
                                as i32,
                        }
                        .into(),
                    );

                    debug_assert_eq!(ctx.current_rsp, beg_rsp, "rsp unchanged after jumping out");
                }

                if el.is_some() {
                    ops.extend(else_ops);
                }
                ops
            }
            Statement::ExpressionStatement { expression } => {
                let mut ops = expression.visit(ctx);
                if let Some(expr_type) = expression.get_type(ctx) {
                    discard_value(ctx, &mut ops, expr_type);
                }
                ops
            }
        }
    }
}

fn calculate_cumsum<'a, I>(iter: I) -> Vec<usize>
where
    I: Iterator<Item = &'a Vec<OperationPrototype>>,
{
    once(0)
        .chain(iter.map(|x| x.len()).scan(0, |sum, i| {
            *sum += i;
            Some(*sum)
        }))
        .collect::<Vec<_>>()
}

fn discard_value(
    ctx: &mut Context,
    ops: &mut Vec<OperationPrototype>,
    return_type: crate::ast::Type,
) {
    let pop_op = match return_type {
        crate::ast::Type::Int => Operation::Pop,
        crate::ast::Type::ArrInt => Operation::HeapPopPtr,
    }
    .into();
    add_operation(ctx, ops, pop_op);
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
                        .map(Some) // TODO: add explicit void type
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
                            arg_count: arg_count as isize,
                        },
                    );
                    ops
                } else {
                    panic!("func '{}' not found", identifier);
                }
            }
            Expression::UnaryMinus { val } => {
                let mut ops = Vec::new();
                ops.extend(val.visit(ctx));

                add_operation(ctx, &mut ops, Operation::Neg.into());

                ops
            }
            Expression::Literal { val } => {
                let mut ops = Vec::new();
                add_operation(ctx, &mut ops, Operation::PushLiteral { value: *val }.into());
                ops
            }
            Expression::VarReference { identifier } => {
                let mut ops = Vec::new();

                if let Some(var_meta) = ctx.get_var(identifier) {
                    match var_meta.type_ {
                        crate::ast::Type::Int => add_operation(
                            ctx,
                            &mut ops,
                            Operation::LocalCopy {
                                variable_offset: ctx.current_rsp as i32
                                    - 1
                                    - var_meta.address as i32,
                            }
                            .into(),
                        ),
                        crate::ast::Type::ArrInt => todo!("Expression::VarReference ArrInt"),
                    }
                } else {
                    panic!("VarReference: var '{}' not found", identifier);
                }

                ops
            }
        }
    }

    fn get_type(&self, ctx: &mut Context) -> Option<crate::ast::Type> {
        match self {
            Expression::Mul { .. }
            | Expression::Div { .. }
            | Expression::Add { .. }
            | Expression::Sub { .. }
            | Expression::Less { .. }
            | Expression::Greater { .. }
            | Expression::LessEq { .. }
            | Expression::GreaterEq { .. }
            | Expression::Eq { .. }
            | Expression::NonEq { .. } => Some(crate::ast::Type::Int),
            Expression::ArrayAccess { .. } => {
                // NOTE: for now only ints can be stored in array
                Some(crate::ast::Type::Int)
            }
            Expression::FuncCall {
                identifier,
                arguments: _,
            } => {
                if let Some(fn_meta) = ctx.get_func(identifier) {
                    fn_meta.return_type
                } else {
                    panic!("FuncCall: func '{}' not found", identifier);
                }
            }
            Expression::UnaryMinus { .. } => Some(crate::ast::Type::Int),
            Expression::Literal { .. } => Some(crate::ast::Type::Int),
            Expression::VarReference { identifier } => {
                if let Some(var_meta) = ctx.get_var(identifier) {
                    Some(var_meta.type_)
                } else {
                    panic!("VarReference: var '{}' not found", identifier);
                }
            }
        }
    }

    fn get_operation(&self) -> Operation {
        match self {
            Expression::Mul { .. } => Operation::Mul,
            Expression::Div { .. } => Operation::Div,
            Expression::Add { .. } => Operation::Add,
            Expression::Sub { .. } => Operation::Sub,
            Expression::Less { .. } => Operation::LessThan,
            Expression::Greater { .. } => Operation::GreaterThan,
            Expression::LessEq { .. } => Operation::LessThanOrEqual,
            Expression::GreaterEq { .. } => Operation::GreaterThanOrEqual,
            Expression::Eq { .. } => Operation::Equal,
            Expression::NonEq { .. } => Operation::NotEqual,
            Expression::UnaryMinus { .. } => Operation::Not,
            other => panic!("no operaton for '{:?}'", other),
        }
    }
}

fn eval_arguments(ctx: &mut Context, ops: &mut Vec<OperationPrototype>, arguments: &[Expression]) {
    ops.extend(arguments.iter().flat_map(|x| x.visit(ctx)));
}

fn handle_builtins(
    ctx: &mut Context,
    ops: &mut Vec<OperationPrototype>,
    identifier: &Identifier,
    arguments: &[Expression],
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
