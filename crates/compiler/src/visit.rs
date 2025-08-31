#![allow(dead_code)]

use std::collections::HashMap;

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

    fn destruct_scope_vars(&mut self) {
        let mut vs = self.cur_scope().declared_vars.iter().collect::<Vec<_>>();
        vs.sort_by(
            |(_, meta1), (_, meta2)| meta1.address.cmp(&meta2.address), /* TODO: check order */
        );

        vs.iter().for_each(|x| match x.1.type_ {
            crate::ast::Type::Int => todo!(),
            crate::ast::Type::ArrInt => todo!(),
        });
    }

    fn exit_scope(&mut self) {
        self.current_rsp = self.scopes.pop().unwrap().start_rsp;
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
