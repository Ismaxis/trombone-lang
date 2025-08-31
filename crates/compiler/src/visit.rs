#![allow(dead_code)]

use std::collections::HashMap;

use crate::error::Result;

use trombone_common::bytecode::Operation;

use crate::{
    ast::{FuncDeclaration, Identifier, Statement},
    instruction_writer::InstructionWriter,
};

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

    // This method pushes operations that destroy variables
    fn destruct_scope_vars(&mut self) {
        let mut vs = self.cur_scope().declared_vars.iter().collect::<Vec<_>>();
        vs.sort_by(
            |(_, meta1), (_, meta2)| meta1.address.cmp(&meta2.address), /* TODO: check order */
        );

        let ops = vs
            .iter()
            .map(|x| match x.1.type_ {
                crate::ast::Type::Int => Operation::Pop,
                crate::ast::Type::ArrInt => Operation::HeapPopPtr,
            })
            .collect::<Vec<_>>();

        ops.iter().for_each(|op| {
            self.write_instruction(*op).unwrap();
        });
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
        debug_assert!(self.declared_vars.insert(id, meta).is_none());
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

struct VarMeta {
    address: usize,
    type_: crate::ast::Type,
}

impl FuncDeclaration {
    pub fn visit(&self, ctx: &mut Context) -> Vec<Operation> {
        // prep
        ctx.declare_func(self.identifier.clone(), FuncMeta {});
        ctx.enter_scope(ScopeTag::Func);

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
            .map(|x| x.visit())
            .flatten()
            .collect::<Vec<Operation>>();

        // DEBUG

        return ops;
    }
}

impl Statement {
    pub fn visit(&self) -> Vec<Operation> {
        Vec::new()
    }
}
