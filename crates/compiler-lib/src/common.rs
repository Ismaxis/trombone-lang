use crate::*;

pub fn define_buildin_funcs(ctx: &mut visit::Context) {
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
