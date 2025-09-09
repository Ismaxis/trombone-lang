pub mod ast;
pub mod error;
pub mod instruction_writer;
pub mod visit;

pub mod common;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub trombone);

use trombone_common::bytecode::Operation;
pub fn compile_from_string(program: String) -> Vec<Operation> {
    let res = trombone::ProgramParser::new().parse(program.as_str());
    assert!(res.is_ok());

    let res: Vec<ast::FuncDeclaration> = res.unwrap();
    assert_eq!(res.len(), 1);
    let res = res[0].clone();

    let mut ctx = visit::Context::new();
    common::define_buildin_funcs(&mut ctx);
    let ops = res.visit(&mut ctx);

    let ops = ops
        .iter()
        .map(|x| match x {
            visit::OperationPrototype::Defined(operation) => operation,
            visit::OperationPrototype::Call { identifier: _ } => todo!(),
            visit::OperationPrototype::Jump => todo!(),
            visit::OperationPrototype::JumpIf => todo!(),
            visit::OperationPrototype::JumpIfNot => todo!(),
            visit::OperationPrototype::SetRetAddress { operands_count: _ } => todo!(),
        })
        .cloned()
        .collect::<Vec<_>>();

    ops
}
