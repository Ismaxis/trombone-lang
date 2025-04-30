use std::collections::BTreeMap;

mod errors;

use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::IntType;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use trombone_common::{TrombValue, bytecode::Operation};

type Rsp = *mut TrombValue;
pub type VmExecuteFunc = unsafe extern "C" fn(Rsp) -> Rsp;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

struct VirtualStack<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    stack_ptr: PointerValue<'ctx>,
    current_offset: i64,
    values: BTreeMap<i64, IntValue<'ctx>>,
}

impl<'ctx> VirtualStack<'ctx> {
    fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        stack_ptr: PointerValue<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            stack_ptr,
            current_offset: 0,
            values: BTreeMap::new(),
        }
    }

    fn get_offset(&self) -> i64 {
        self.current_offset
    }

    fn push(&mut self, value: IntValue<'ctx>) {
        self.values.insert(self.current_offset, value);
        self.current_offset += 1;
    }

    fn pop(&mut self) -> IntValue<'ctx> {
        self.current_offset -= 1;
        self.get(0)
    }

    fn set(&mut self, offset: i64, value: IntValue<'ctx>) {
        self.values.insert(self.current_offset + offset, value);
    }

    fn get(&mut self, offset: i64) -> IntValue<'ctx> {
        let offset = self.current_offset + offset;
        if let Some(v) = self.values.get(&offset) {
            *v
        } else {
            let i64_type = self.context.i64_type();
            let ptr = ptr_with_offset(
                offset,
                "ptr_with_offset_pop",
                self.stack_ptr,
                i64_type,
                self.builder,
            );
            let value = self
                .builder
                .build_load(i64_type, ptr, "pop_value")
                .expect("pop value")
                .into_int_value();
            self.values.insert(offset, value);
            value
        }
    }

    fn finalize(&mut self) {
        let i64_type = self.context.i64_type();
        for (offset, value) in self.values.iter() {
            if *offset >= self.current_offset {
                return;
            }
            let ptr = ptr_with_offset(
                *offset,
                "ptr_with_offset_finalize",
                self.stack_ptr,
                i64_type,
                self.builder,
            );
            self.builder
                .build_store(ptr, *value)
                .expect("update stack finalize");
        }
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            execution_engine,
        }
    }

    pub fn jit_compile_basic_block(
        &self,
        block_id: usize,
        operations: &[Operation],
    ) -> errors::Result<JitFunction<VmExecuteFunc>> {
        let module_name = format!("block_module_{}", block_id);
        let module = self.context.create_module(&module_name);

        let state_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
        let ret_type = state_ptr_type;

        let fn_type = ret_type.fn_type(&[state_ptr_type.into()], false);
        let function = module.add_function(&module_name, fn_type, None);
        let rsp = function.get_nth_param(0).unwrap().into_pointer_value();

        let continuous_block_operations: Vec<_> = operations
            .split_inclusive(|x| matches!(*x, Operation::Div | Operation::Mod))
            .collect();
        let mut basic_blocks = Vec::new();
        let block_count = continuous_block_operations.len() + 2; // blocks + return in case of success + return in case of fail
        basic_blocks.reserve(block_count);
        for idx in 0..(block_count) {
            basic_blocks.push(
                self.context
                    .append_basic_block(function, &format!("basic_block_{}_{}", block_id, idx)),
            )
        }

        let mut vstack = VirtualStack::new(self.context, &self.builder, rsp);
        for (idx, ops) in continuous_block_operations.iter().enumerate() {
            self.builder.position_at_end(basic_blocks[idx]);
            self.compile_basic_block(
                ops,
                &mut vstack,
                &basic_blocks[idx + 1],
                basic_blocks.last().unwrap(),
            );
        }

        {
            // success
            self.builder
                .position_at_end(basic_blocks[basic_blocks.len() - 2]);
            vstack.finalize();

            let result = BasicValueEnum::PointerValue(self.ptr_with_offset(
                vstack.get_offset(),
                "new_stack_ptr",
                rsp,
            ));

            self.builder.build_return(Some(&result)).unwrap();
        }

        {
            // fail
            self.builder
                .position_at_end(basic_blocks[basic_blocks.len() - 1]);

            let result = ret_type.const_null();

            self.builder.build_return(Some(&result)).unwrap();
        }

        function
            .verify(true)
            .then_some(())
            .ok_or("VerificationError".to_string())?;

        self.execution_engine.add_module(&module).unwrap();

        unsafe {
            self.execution_engine
                .get_function(&module_name)
                .map_err(|err| format!("GetFunctionError: {}", err).into())
        }
    }

    fn compile_basic_block<'s>(
        &'s self,
        operations: &[Operation],
        vstack: &mut VirtualStack<'s>,
        next_block: &BasicBlock,
        error_block: &BasicBlock,
    )
    // -> BasicValueEnum<'ctx>
    {
        for op in operations {
            match op {
                // Stack operations
                Operation::PushLiteral { value } => {
                    vstack.push(self.context.i64_type().const_int(*value as u64, false));
                }
                Operation::Pop => {
                    vstack.pop();
                }
                Operation::LocalCopy { variable_offset } => {
                    let value = vstack.get(calc_stack_offset(variable_offset));
                    vstack.push(value);
                }
                Operation::LocalStore { variable_offset } => {
                    let value = vstack.pop();
                    vstack.set(calc_stack_offset(variable_offset), value);
                }

                // Arithmetic
                Operation::Neg => self.unary_op(vstack, Builder::build_int_neg, "neg"),
                Operation::Not => self.unary_op(vstack, Builder::build_not, "not"),

                Operation::Add => self.binary_op(vstack, Builder::build_int_add, "add"),
                Operation::Sub => self.binary_op(vstack, Builder::build_int_sub, "sub"),
                Operation::Mul => self.binary_op(vstack, Builder::build_int_mul, "mul"),
                Operation::Div => {
                    self.errorneous_binary_op(
                        vstack,
                        Builder::build_int_signed_div,
                        "div",
                        next_block,
                        error_block,
                    );
                    return;
                }
                Operation::Mod => {
                    self.errorneous_binary_op(
                        vstack,
                        Builder::build_int_signed_rem,
                        "mod",
                        next_block,
                        error_block,
                    );
                    return;
                }
                Operation::And => self.binary_op(vstack, Builder::build_and, "and"),
                Operation::Or => self.binary_op(vstack, Builder::build_or, "or"),
                Operation::Xor => self.binary_op(vstack, Builder::build_xor, "xor"),
                Operation::Lsh => self.binary_op(vstack, Builder::build_left_shift, "lsh"),
                Operation::Rsh => self.binary_op(
                    vstack,
                    |b: &Builder, lhs, rhs, name| b.build_right_shift(lhs, rhs, true, name),
                    "rsh",
                ),

                // Comparison
                Operation::Equal => {
                    self.comparison(vstack, IntPredicate::EQ);
                }
                Operation::NotEqual => {
                    self.comparison(vstack, IntPredicate::NE);
                }
                Operation::LessThan => {
                    self.comparison(vstack, IntPredicate::SLT);
                }
                Operation::GreaterThan => {
                    self.comparison(vstack, IntPredicate::SGT);
                }
                Operation::LessThanOrEqual => {
                    self.comparison(vstack, IntPredicate::SLE);
                }
                Operation::GreaterThanOrEqual => {
                    self.comparison(vstack, IntPredicate::SGE);
                }

                _ => {
                    panic!("unsupported operation");
                }
            }
        }

        self.builder
            .build_unconditional_branch(*next_block)
            .unwrap();
    }

    fn ptr_with_offset(
        &self,
        offset: i64,
        name: &str,
        current_ptr: PointerValue<'ctx>,
    ) -> PointerValue<'ctx> {
        ptr_with_offset(
            offset,
            name,
            current_ptr,
            self.context.i64_type(),
            &self.builder,
        )
    }

    fn comparison<'s>(&'s self, vstack: &mut VirtualStack<'s>, op: IntPredicate) {
        let rhs = vstack.pop();
        let lhs = vstack.pop();

        let eq = self
            .builder
            .build_int_compare(op, lhs, rhs, "cmp_result")
            .unwrap();
        let eq = self
            .builder
            .build_int_z_extend(eq, self.context.i64_type(), "")
            .unwrap();
        vstack.push(eq);
    }

    fn unary_op<'s, F>(&'s self, vstack: &mut VirtualStack<'s>, op: F, unary_str: &str)
    where
        F: FnOnce(&Builder<'s>, IntValue<'s>, &str) -> Result<IntValue<'s>, BuilderError>,
    {
        let value = vstack.pop();
        let result = op(&self.builder, value, unary_str).unwrap();
        vstack.push(result);
    }

    fn binary_op<'s, F>(&'s self, vstack: &mut VirtualStack<'s>, op: F, binary_str: &str)
    where
        F: FnOnce(
            &Builder<'s>,
            IntValue<'s>,
            IntValue<'s>,
            &str,
        ) -> Result<IntValue<'s>, BuilderError>,
    {
        let rhs = vstack.pop();
        let lhs = vstack.pop();
        let result = op(&self.builder, lhs, rhs, binary_str).unwrap();
        vstack.push(result);
    }

    fn errorneous_binary_op<'s, F>(
        &'s self,
        vstack: &mut VirtualStack<'s>,
        op: F,
        binary_str: &str,
        next_block: &BasicBlock,
        fail_block: &BasicBlock,
    ) where
        F: FnOnce(
            &Builder<'s>,
            IntValue<'s>,
            IntValue<'s>,
            &str,
        ) -> Result<IntValue<'s>, BuilderError>,
    {
        let rhs = vstack.pop();
        let lhs = vstack.pop();
        let eq = self
            .builder
            .build_int_compare(
                IntPredicate::NE,
                rhs,
                self.context.i64_type().const_zero(),
                "cmp_result",
            )
            .unwrap();
        self.builder
            .build_conditional_branch(eq, *next_block, *fail_block)
            .unwrap();
        self.builder.position_at_end(*next_block);
        let result = op(&self.builder, lhs, rhs, binary_str).unwrap();
        vstack.push(result);
    }

    pub fn jit_compile_basic_block(
        &self,
        block_id: usize,
        operations: &[Operation],
    ) -> Option<JitFunction<VmExecuteFunc>> {
        let module_name = format!("block_module_{}", block_id);
        let module = self.context.create_module(&module_name);

        let state_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
        let ret_type = state_ptr_type;

        let fn_type = ret_type.fn_type(&[state_ptr_type.into()], false);
        let function = module.add_function(&module_name, fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        let rsp = function.get_nth_param(0)?.into_pointer_value();

        let result = self.compile_operations(operations, rsp)?;
        self.builder.build_return(Some(&result)).ok()?;

        function.verify(true).then_some(())?;

        self.execution_engine.add_module(&module).ok()?;
        eprintln!("{}", module.print_to_string());

        unsafe { self.execution_engine.get_function(&module_name).ok() }
    }

    fn compile_operations(
        &self,
        operations: &[Operation],
        rsp: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> /* TODO: result */ {
        let i64_type = self.context.i64_type();
        let stack_ptr = self
            .builder
            .build_alloca(rsp.get_type(), "stack_ptr_var")
            .ok()?;
        self.builder.build_store(stack_ptr, rsp).ok()?;

        for op in operations {
            let current_stack_ptr = self.get_current_stack_pointer(rsp, stack_ptr)?;
            match op {
                Operation::PushLiteral { value } => {
                    let const_val = i64_type.const_int(*value as u64, false);
                    self.builder
                        .build_store(current_stack_ptr, const_val)
                        .ok()?;

                    self.update_stack_pointer(1, stack_ptr, current_stack_ptr);
                }
                Operation::Pop => {
                    self.update_stack_pointer(-1, stack_ptr, current_stack_ptr)?;
                }
                Operation::LocalCopy { variable_offset } => {
                    let offset = -*variable_offset - 1;
                    let variable_ptr =
                        self.ptr_with_offset(offset as i64, "variable_to_copy", current_stack_ptr)?;
                    let variable_value = self
                        .builder
                        .build_load(variable_ptr.get_type(), variable_ptr, "value_to_copy")
                        .ok()?;
                    self.builder
                        .build_store(current_stack_ptr, variable_value)
                        .ok()?;
                    self.update_stack_pointer(1, stack_ptr, current_stack_ptr)?;
                }
                Operation::LocalStore { variable_offset } => {
                    // TODO: optimize
                    let offset = -*variable_offset - 1;
                    let moved_stack_ptr =
                        self.update_stack_pointer(-1, stack_ptr, current_stack_ptr)?;
                    let value = self
                        .builder
                        .build_load(
                            moved_stack_ptr.get_type(),
                            moved_stack_ptr,
                            "value_to_store",
                        )
                        .ok()?;
                    let variable_ptr =
                        self.ptr_with_offset(offset as i64, "variable_to_update", moved_stack_ptr)?;
                    self.builder.build_store(variable_ptr, value).ok()?;
                }

                // Arithmetic
                // ...

                // Comparison
                Operation::Equal => {
                    let lhs = self.stack_get(-1, current_stack_ptr)?;
                    let rhs = self.stack_get(-2, current_stack_ptr)?;

                    assert!(lhs.is_int_value()); // TODO: return Err
                    assert!(rhs.is_int_value()); // TODO: return Err

                    let eq = self
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::EQ,
                            lhs.into_int_value(),
                            rhs.into_int_value(),
                            "eq_result",
                        )
                        .ok()?;

                    self.stack_put(-2, current_stack_ptr, eq)?;
                    self.update_stack_pointer(-1, stack_ptr, current_stack_ptr)?;
                }
                // ...

                // break on non-supported operations
                _ => {
                    return None;
                }
            }
        }

        let final_stack_ptr = self
            .builder
            .build_load(rsp.get_type(), stack_ptr, "final_stack_ptr")
            .ok()?;
        Some(final_stack_ptr)
    }

    fn stack_put(
        &self,
        offset: i64,
        current_stack_ptr: PointerValue<'ctx>,
        value: IntValue<'_>,
    ) -> Option<()> /* TODO: result */ {
        self.builder
            .build_store(
                self.ptr_with_offset(offset, "store_stack_ptr", current_stack_ptr)?,
                value,
            )
            .ok()
            .map(|_| ())
    }

    fn stack_get(
        &self,
        offset: i64,
        current_stack_ptr: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> /* TODO: result */ {
        self.builder
            .build_load(
                self.context.i64_type(),
                self.ptr_with_offset(offset, "load_stack_ptr", current_stack_ptr)?,
                "value_on_stack",
            )
            .ok()
    }

    fn get_current_stack_pointer(
        &self,
        rsp: PointerValue<'ctx>,
        stack_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        let current_stack_ptr = self
            .builder
            .build_load(rsp.get_type(), stack_ptr, "current_stack_ptr")
            .ok()?
            .into_pointer_value();
        Some(current_stack_ptr)
    }

    fn update_stack_pointer(
        &self,
        offset: i64,
        stack_ptr: PointerValue<'ctx>,
        current_stack_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        let new_stack_ptr = self.ptr_with_offset(offset, "new_stack_ptr", current_stack_ptr)?;
        self.builder
            .build_store(stack_ptr, new_stack_ptr)
            .ok()
            .map(|_| new_stack_ptr)
    }

    fn ptr_with_offset(
        &self,
        offset: i64,
        name: &str,
        current_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        let i64_type = self.context.i64_type();
        let offset_const = if offset < 0 {
            i64_type.const_int((-offset) as u64, true).const_neg()
        } else {
            i64_type.const_int(offset as u64, false)
        };
        unsafe {
            self.builder
                .build_in_bounds_gep(i64_type, current_ptr, &[offset_const], name)
        }
        .ok()
    }

    #[allow(dead_code)]
    pub fn inspect_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

fn calc_stack_offset(variable_offset: &i32) -> i64 {
    -(*variable_offset as i64) - 1
}

fn ptr_with_offset<'ctx>(
    offset: i64,
    name: &str,
    current_ptr: PointerValue<'ctx>,
    i64_type: IntType<'ctx>,
    builder: &Builder<'ctx>,
) -> PointerValue<'ctx> {
    let offset_const = if offset < 0 {
        i64_type.const_int((-offset) as u64, true).const_neg()
    } else {
        i64_type.const_int(offset as u64, false)
    };
    unsafe { builder.build_in_bounds_gep(i64_type, current_ptr, &[offset_const], name) }.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::OptimizationLevel;
    use trombone_common::TrombValue;

    // https://stackoverflow.com/a/52843365/17826620
    #[test]
    fn test_jit_push_pop() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        // Push
        let push_literal_jit = codegen
            .jit_compile_basic_block(0, &[Operation::PushLiteral { value: 42 }])
            .expect("Failed to compile push function");
        {
            let modified_stack_ptr = unsafe { push_literal_jit.call(stack_base) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 1));
            assert_eq!(stack[..1], [42]);
            assert_eq!(stack[1..], [0; 15]);
        }

        // Pop
        let pop_jit = codegen
            .jit_compile_basic_block(1, &[Operation::Pop])
            .expect("Failed to compile pop function");
        {
            let modified_stack_ptr = unsafe { pop_jit.call(offset_ptr(stack_base, 1)) };

            assert_eq!(modified_stack_ptr, stack_base);
        }
    }

    #[test]
    fn test_jit_stack_operations() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        // LocalCopy
        let localcopy_jit = codegen
            .jit_compile_basic_block(0, &[Operation::LocalCopy { variable_offset: 7 }; 3])
            .expect("Failed to compile local_copy function");
        {
            stack[0] = 42;
            stack[1] = 54;
            stack[2] = 68;

            let modified_stack_ptr = unsafe { localcopy_jit.call(offset_ptr(stack_base, 8)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 11));
            assert_eq!(stack[..11], [42, 54, 68, 0, 0, 0, 0, 0, 42, 54, 68]);
        }

        // LocalStore
        let localstore_jit = codegen
            .jit_compile_basic_block(1, &[Operation::LocalStore { variable_offset: 7 }; 3])
            .expect("Failed to compile local_store function");
        {
            stack[8] = 11;
            stack[9] = 12;
            stack[10] = 13;

            let modified_stack_ptr = unsafe { localstore_jit.call(offset_ptr(stack_base, 11)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 8));
            assert_eq!(stack[..8], [11, 12, 13, 0, 0, 0, 0, 0]);
        }
    }

    #[test]
    fn test_jit_advanced_stack_operations() {
        let context = Context::create();
        let codegen = init(&context);

        let localcopy_jit_zero = codegen
            .jit_compile_basic_block(0, &[Operation::LocalCopy { variable_offset: 0 }])
            .expect("Failed to compile local_copy function");
        let localstore_jit_zero = codegen
            .jit_compile_basic_block(1, &[Operation::LocalStore { variable_offset: 0 }])
            .expect("Failed to compile local_store function");

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        {
            stack[0] = 42;

            let modified_stack_ptr = unsafe { localcopy_jit_zero.call(offset_ptr(stack_base, 1)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 2));
            assert_eq!(stack[..2], [42, 42]);
        }
        {
            stack[2] = 1000;

            let modified_stack_ptr = unsafe { localstore_jit_zero.call(offset_ptr(stack_base, 3)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 2));
            assert_eq!(stack[..2], [42, 1000]);
        }
    }

    #[test]
    fn test_jit_cmp() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        let eq_jit = codegen
            .jit_compile_basic_block(0, &[Operation::Equal])
            .expect("Failed to compile equal function");
        let not_eq_jit = codegen
            .jit_compile_basic_block(1, &[Operation::NotEqual])
            .expect("Failed to compile not_equal function");
        let lt_jit = codegen
            .jit_compile_basic_block(2, &[Operation::LessThan])
            .expect("Failed to compile less function");
        let gt_jit = codegen
            .jit_compile_basic_block(3, &[Operation::GreaterThan])
            .expect("Failed to compile greater function");
        let le_jit = codegen
            .jit_compile_basic_block(4, &[Operation::LessThanOrEqual])
            .expect("Failed to compile less_or_equal function");
        let ge_jit = codegen
            .jit_compile_basic_block(5, &[Operation::GreaterThanOrEqual])
            .expect("Failed to compile greater_or_equal function");

        let tests = [
            ("5 == 5 -> 1", &eq_jit, [5, 5], [1]),
            ("5 == 3 -> 0", &eq_jit, [5, 3], [0]),
            ("5 != 5 -> 0", &not_eq_jit, [5, 5], [0]),
            ("5 != 3 -> 1", &not_eq_jit, [5, 3], [1]),
            ("3 < 5 -> 1", &lt_jit, [3, 5], [1]),
            ("5 < 3 -> 0", &lt_jit, [5, 3], [0]),
            ("5 < 5 -> 0", &lt_jit, [5, 5], [0]),
            ("3 > 5 -> 0", &gt_jit, [3, 5], [0]),
            ("5 > 3 -> 1", &gt_jit, [5, 3], [1]),
            ("5 > 5 -> 0", &gt_jit, [5, 5], [0]),
            ("3 <= 5 -> 1", &le_jit, [3, 5], [1]),
            ("5 <= 3 -> 0", &le_jit, [5, 3], [0]),
            ("5 <= 5 -> 1", &le_jit, [5, 5], [1]),
            ("3 >= 5 -> 0", &ge_jit, [3, 5], [0]),
            ("5 >= 3 -> 1", &ge_jit, [5, 3], [1]),
            ("5 >= 5 -> 1", &ge_jit, [5, 5], [1]),
        ];

        for (name, func, initial_stack, final_stack) in tests {
            for (i, v) in initial_stack.iter().enumerate() {
                stack[i] = *v;
            }

            let modified_stack_ptr = unsafe { func.call(offset_ptr(stack_base, 2)) };

            assert_eq!(modified_stack_ptr, offset_ptr(stack_base, 1), "{name}");
            assert_eq!(stack[..final_stack.len()], final_stack, "{name}");
        }
    }

    #[test]
    fn test_jit_bool_to_int_cast() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        let bool_cast_jit = codegen
            .jit_compile_basic_block(0, &[Operation::Equal, Operation::Mul])
            .expect("Failed to compile function");

        let tests = [
            ("(5 == 5) * 10 -> 10", [10, 5, 5], [10]),
            ("(5 == 3) * 10 -> 0", [10, 3, 5], [0]),
        ];

        for (name, initial_stack, final_stack) in tests {
            for (i, v) in initial_stack.iter().enumerate() {
                stack[i] = *v;
            }

            let modified_stack_ptr =
                unsafe { bool_cast_jit.call(offset_ptr(stack_base, initial_stack.len() as i64)) };

            assert_eq!(
                modified_stack_ptr,
                offset_ptr(stack_base, final_stack.len() as i64),
                "{name}"
            );
            assert_eq!(stack[..final_stack.len()], final_stack, "{name}");
        }
    }

    #[test]
    fn test_jit_unary_arithmetic() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        #[allow(clippy::type_complexity)]
        let tests: [(_, _, Box<dyn Fn(TrombValue) -> TrombValue>); 2] = [
            ("neg", Operation::Neg, Box::new(|x| -x)),
            ("not", Operation::Not, Box::new(std::ops::Not::not)),
        ];

        let lhs = 0;

        for (idx, (op_str, op, f)) in tests.iter().enumerate() {
            stack[0] = lhs;

            let jitted = codegen
                .jit_compile_basic_block(idx, &[*op])
                .unwrap_or_else(|_| panic!("Failed to compile {}", op_str));

            let new_stack_ptr = unsafe { jitted.call(offset_ptr(stack_base, 1)) };
            assert_eq!(stack[0], f(lhs), "{}", op_str);
            assert_eq!(new_stack_ptr, offset_ptr(stack_base, 1));
        }
    }

    #[test]
    fn test_jit_binary_arithmetic() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        #[allow(clippy::type_complexity)]
        let tests: [(_, _, Box<dyn Fn(TrombValue, TrombValue) -> TrombValue>); 10] = [
            ("add", Operation::Add, Box::new(|x, y| x + y)),
            ("sub", Operation::Sub, Box::new(|x, y| x - y)),
            ("mul", Operation::Mul, Box::new(|x, y| x * y)),
            ("div", Operation::Div, Box::new(|x, y| x / y)),
            ("mod", Operation::Mod, Box::new(|x, y| x % y)),
            ("and", Operation::And, Box::new(|x, y| x & y)),
            ("or", Operation::Or, Box::new(|x, y| x | y)),
            ("xor", Operation::Xor, Box::new(|x, y| x ^ y)),
            ("lsh", Operation::Lsh, Box::new(|x, y| x << y)),
            ("rsh", Operation::Rsh, Box::new(|x, y| x >> y)),
        ];

        let lhs = 77;
        let rhs = 4;

        for (idx, (op_str, op, f)) in tests.iter().enumerate() {
            stack[0] = lhs;
            stack[1] = rhs;

            let jitted = codegen
                .jit_compile_basic_block(idx, &[*op])
                .unwrap_or_else(|_| panic!("Failed to compile {}", op_str));

            let new_stack_ptr = unsafe { jitted.call(offset_ptr(stack_base, 2)) };
            assert_eq!(stack[0], f(lhs, rhs), "{}", op_str);
            assert_eq!(new_stack_ptr, offset_ptr(stack_base, 1));
        }
    }

    #[test]
    fn bench_jit_binary_arithmetic() {
        let context = Context::create();
        let codegen = init(&context);

        const N: usize = 1000;

        let operations = Vec::from_iter((0..N - 1).map(|_| Operation::Add));

        let jitted = codegen
            .jit_compile_basic_block(0, &operations)
            .expect("Failed to compile add");

        let mut stack: [TrombValue; N + 1] = [1; N + 1];
        let stack_base = stack.as_mut_ptr();

        let start = std::time::Instant::now();
        let modified = unsafe { jitted.call(offset_ptr(stack_base, N as i64)) };
        let elapsed = start.elapsed();
        assert_eq!(modified, offset_ptr(stack_base, 1));
        assert_eq!(stack[0], N as TrombValue);

        println!("Elapsed time: {:?}", elapsed);
        assert!(elapsed.as_nanos() < 1000);
        // On my machine, jit with virtual stack takes 400ns, while jit with as-is translation takes 1500ns
    }

    #[test]
    fn test_jit_error_handling() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

        let operations = [
            Operation::Add,
            Operation::Add,
            Operation::Div,
            Operation::Sub,
            Operation::Sub,
            Operation::Mod,
            Operation::Mul,
            Operation::Mul,
            Operation::Div,
        ];

        let stacks: Vec<&[i64]> = vec![
            &[1, 0, 0, 0],
            &[1, 0, 0, 1, 0, 1, 1],
            &[1, 0, 0, 1, 0, 1, 1, 0, 1, 1],
        ];

        for (idx, new_stack) in stacks.iter().enumerate() {
            for (i, val) in new_stack.iter().enumerate() {
                stack[i] = *val;
            }

            let jitted = codegen
                .jit_compile_basic_block(
                    idx,
                    &operations[..std::cmp::min(idx * 3 + 4, operations.len())],
                )
                .unwrap_or_else(|_| panic!("Failed to compile {}", idx));

            let new_stack_ptr =
                unsafe { jitted.call(offset_ptr(stack_base, new_stack.len() as i64)) };
            assert_eq!(new_stack_ptr, std::ptr::null_mut(), "test {}", idx);
        }
    }

    fn init(context: &Context) -> CodeGen {
        let module = context.create_module("unused_module");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Failed to create JIT execution engine");

        let codegen = CodeGen::new(context, module, builder, execution_engine);
        codegen
    }

    fn offset_ptr(ptr: Rsp, offset: i64) -> Rsp {
        (ptr as i64 + offset * std::mem::size_of::<Rsp>() as i64) as Rsp
    }
}
