use std::collections::HashMap;

use crate::*;

use trombone_common::bytecode::Operation;

pub fn compile_from_string(program: String) -> Vec<Operation> {
    let parsed = trombone::ProgramParser::new().parse(program.as_str());
    assert!(parsed.is_ok());

    let funcs_ast = parsed.unwrap();

    let mut ctx = visit::Context::new();
    compiler::define_builtin_funcs(&mut ctx);

    let mut global_ops = vec![];

    let mut global_file_offset = 0;
    funcs_ast.iter().for_each(|f| {
        ctx.declare_func(
            &f.identifier,
            visit::FuncMeta {
                arguments_types: f.params.iter().map(|x| x.type_).collect(),
                return_type: f.return_type,
            },
        );
    });

    let compiled_func = funcs_ast.iter().map(|f| f.visit(&mut ctx));
    let funcs_addresses = compiled_func
        .enumerate()
        .map(|(i, ops)| {
            let func_address = global_file_offset;

            global_file_offset += ops.len();
            global_ops.extend(ops);

            (funcs_ast[i].identifier.clone(), func_address as i32)
        })
        .collect::<HashMap<_, _>>();

    let global_ops = global_ops
        .iter()
        .enumerate()
        .rev() // this is needed for basicBlockStart to calculate basic block len
        .map(|(i, x)| match x {
            visit::OperationPrototype::Defined(operation) => operation.clone(),
            visit::OperationPrototype::Call { identifier, arg_count: _ } => {
                if let Some(func_address) = funcs_addresses.get(identifier) {
                    return Operation::Jump {
                        offset: *func_address - (i as i32),
                    };
                } else {
                    panic!("OperationPrototype::Call: func '{}' not found", identifier);
                }
            }
            visit::OperationPrototype::BasicBlockStart => {
                todo!("visit::OperationPrototype::BasicBlockStart")
            }
        })
        .rev()
        .collect::<Vec<_>>();

    global_ops
}

pub fn define_builtin_funcs(ctx: &mut visit::Context) {
    ctx.declare_func(
        &"print".to_string(),
        visit::FuncMeta {
            arguments_types: vec![ast::Type::Int],
            return_type: None,
        },
    );

    ctx.declare_func(
        &"read".to_string(),
        visit::FuncMeta {
            arguments_types: vec![],
            return_type: Some(ast::Type::Int),
        },
    );

    ctx.declare_func(
        &"array".to_string(),
        visit::FuncMeta {
            arguments_types: vec![ast::Type::Int],
            return_type: Some(ast::Type::ArrInt),
        },
    );
}
