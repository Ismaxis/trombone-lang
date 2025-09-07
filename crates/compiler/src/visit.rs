#![allow(dead_code)]

use std::collections::HashMap;

use crate::{ast::Expression, error::Result};

use trombone_common::bytecode::Operation;

use crate::{
    ast::{FuncDeclaration, Identifier, Statement},
    instruction_writer::InstructionWriter,
};

#[derive(Clone)]
enum NotCompletedOperation {
    Completed(Operation),
    Jump,
    ConditionalJump,
}

impl NotCompletedOperation {
    fn calc_stack_diff(&self) -> isize {
        let unused_value = 1337;
        match self {
            Self::Completed(op) => op.calc_stack_diff(),
            Self::Jump => Operation::Jump {
                offset: unused_value,
            }
            .calc_stack_diff(),
            Self::ConditionalJump => Operation::JumpIf {
                offset: unused_value,
            }
            .calc_stack_diff(),
        }
    }
}

impl From<Operation> for NotCompletedOperation {
    fn from(value: Operation) -> Self {
        Self::Completed(value)
    }
}

fn add_operation(
    ops: &mut Vec<NotCompletedOperation>,
    op: NotCompletedOperation,
    ctx: &mut Context,
) {
    let stack_diff = op.calc_stack_diff();
    ops.push(op.clone().into());
    ctx.current_rsp = (ctx.current_rsp as isize + stack_diff) as usize;
}

pub struct Context {
    current_rsp: usize,
    declared_funcs: HashMap<Identifier, FuncMeta>,
    scopes: Vec<Scope>,

    writer: InstructionWriter<Vec<u8>>,
}

impl Context {
    fn new(writer: InstructionWriter<Vec<u8>>) -> Self {
        Self {
            current_rsp: 0,
            declared_funcs: HashMap::new(),
            scopes: Vec::new(),
            writer,
        }
    }

    fn declare_func(&mut self, id: Identifier, meta: FuncMeta) {
        self.declared_funcs.insert(id, meta);
    }

    fn get_var(&mut self, id: Identifier) -> Option<VarMeta> {
        for scope in self.scopes.iter().rev() {
            let opMeta = scope.declared_vars.get(&id);
            if opMeta.is_some() {
                return opMeta.cloned();
            }
        }

        None
        // let res = self.declared_vars.insert(id, meta);
        // debug_assert!(res.is_none());
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
    fn destruct_scope_vars(&mut self) -> Vec<NotCompletedOperation> {
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

    fn destruct_all_vars(&mut self) -> Vec<NotCompletedOperation> {
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

    fn write_instruction(&mut self, op: Operation) -> Result<usize> {
        self.current_rsp = (self.current_rsp as isize + op.calc_stack_diff()) as usize;
        self.writer.write(op)
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

struct FuncMeta {
    //
}

#[derive(Clone)]
struct VarMeta {
    address: usize, // rsp at the moment of declaration
    type_: crate::ast::Type,
}

// Impl

impl FuncDeclaration {
    pub fn visit(&self, ctx: &mut Context) -> Vec<NotCompletedOperation> {
        debug_assert_eq!(ctx.current_rsp, 0, "no stack at the beggining of the func");

        // prep
        ctx.declare_func(self.identifier.clone(), FuncMeta {});
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

        // operands are cleared in return // ReturnStatement is essential
        debug_assert!(matches!(
            ops.last(),
            Some(NotCompletedOperation::Completed(Operation::Return))
        ));

        return ops;
    }
}

impl Statement {
    pub fn visit(&self, ctx: &mut Context) -> Vec<NotCompletedOperation> {
        match self {
            Statement::VarDeclaration {
                identifier,
                type_,
                value,
            } => {
                let ops = value.visit(ctx);
                let address = ctx.current_rsp;
                ctx.cur_scope().declare_var(
                    identifier.clone(),
                    VarMeta {
                        address,
                        type_: *type_,
                    },
                );
                ctx.current_rsp += 1; // This is one of the few places where current rsp moves manually, not by operations
                return ops;
            }
            Statement::Assignment { identifier, value } => {
                if let Some(var_meta) = ctx.get_var(identifier.clone()) {
                    let mut ops = value.visit(ctx);
                    match var_meta.type_ {
                        crate::ast::Type::Int => {
                            let offset = ctx.current_rsp - 1 - var_meta.address - 1; // TODO: check
                            let op = Operation::LocalStore {
                                variable_offset: offset as i32,
                            };
                            add_operation(&mut ops, op.into(), ctx);
                            return ops;
                        }
                        crate::ast::Type::ArrInt => todo!("Not implemented assignment to array"),
                    }
                } else {
                    todo!("variable not found (implement error handling)")
                }
            }
            Statement::ArrayAssignment {
                identifier,
                index,
                value,
            } => todo!(),
            Statement::ReturnStatement { return_value } => {
                let mut ops = Vec::new();
                if let Some(return_value) = return_value {
                    ops = return_value.visit(ctx);
                    let return_value_offset = (ctx.current_rsp - 1) as i32; // TODO: check; abstract calculating it in other method
                    let set_return_value_op = Operation::LocalStore {
                        variable_offset: return_value_offset,
                    };
                    add_operation(&mut ops, set_return_value_op.into(), ctx);
                }
                ops.append(&mut ctx.destruct_all_vars());
                let return_op = Operation::Return;
                add_operation(&mut ops, return_op.into(), ctx);
                ops
            }
            Statement::WhileStatement {
                condition,
                statements,
            } => todo!(),
            Statement::IfStatement { arms, el } => todo!(),
            Statement::ExpressionStatement { expression } => todo!(),
        }
        // Vec::new()
    }
}

impl Expression {
    pub fn visit(&self, _ctx: &mut crate::visit::Context) -> Vec<NotCompletedOperation> {
        todo!()
    }
}
