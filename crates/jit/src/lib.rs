use std::collections::BTreeMap;

mod errors;

use errors::Error;
use inkwell::IntPredicate;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::IntType;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use trombone_common::bytecode::Operation;

type Rsp = *mut u64;
pub type VmExecuteFunc = unsafe extern "C" fn(Rsp) -> Rsp;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

struct VirtualStack<'ctx, 'bldr> {
    context: &'ctx Context,
    builder: &'bldr Builder<'ctx>,
    stack_ptr: PointerValue<'ctx>,
    current_offset: i64,
    values: BTreeMap<i64, IntValue<'ctx>>,
}

impl<'ctx> VirtualStack<'ctx, 'ctx> {
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
        if let Some(v) = self.values.get(&self.current_offset) {
            *v
        } else {
            let i64_type = self.context.i64_type();
            let ptr = ptr_with_offset(
                self.current_offset,
                "ptr_with_offset_pop",
                self.stack_ptr,
                i64_type,
                self.builder,
            )
            .expect("ptr with offset pop");
            let value = self
                .builder
                .build_load(i64_type, ptr, "pop_value")
                .expect("pop value")
                .into_int_value();
            self.values.insert(self.current_offset, value);
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
            )
            .expect("ptr with offset finalize");
            self.builder
                .build_store(ptr, *value)
                .expect("update stack finalize");
        }
    }

    // fn put_ith(&mut self, offset: i64) {
    //     //
    // }

    // fn get_ith(&mut self, offset: i64) -> IntValue<'ctx> {
    //     self.context.i64_type().const_int(228, false)
    // }
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
    ) -> errors::Result<JitFunction<VmExecuteFunc>> /* TODO: result */ {
        let module_name = format!("block_module_{}", block_id);
        let module = self.context.create_module(&module_name);

        let state_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
        let ret_type = state_ptr_type;

        let fn_type = ret_type.fn_type(&[state_ptr_type.into()], false);
        let function = module.add_function(&module_name, fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        let rsp = function.get_nth_param(0).unwrap().into_pointer_value();

        let result = self
            .compile_operations(operations, rsp)
            .ok_or(Error::CompilationError)?;
        self.builder.build_return(Some(&result)).unwrap();

        eprintln!("{}", module.print_to_string()); // TODO: remove
        function
            .verify(true)
            .then_some(())
            .ok_or(Error::VerificationError)?;

        self.execution_engine.add_module(&module).unwrap();

        Ok(unsafe { self.execution_engine.get_function(&module_name).unwrap() })
    }

    fn compile_operations(
        &self,
        operations: &[Operation],
        mut stack_ptr: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> /* TODO: result */ {
        let i64_type = self.context.i64_type();

        let mut vstack = VirtualStack::new(self.context, &self.builder, stack_ptr);

        for op in operations {
            match op {
                // Stack operations
                Operation::PushLiteral { value } => {
                    self.stack_put(0, stack_ptr, i64_type.const_int(*value as u64, false))?;
                    stack_ptr = self.update_stack_pointer(1, stack_ptr)?;
                }
                Operation::Pop => {
                    stack_ptr = self.update_stack_pointer(-1, stack_ptr)?;
                }
                Operation::LocalCopy { variable_offset } => {
                    let offset = calc_stack_offset(variable_offset);
                    let value = self.stack_get(offset, stack_ptr)?;
                    self.stack_put(0, stack_ptr, value.into_int_value())?;
                    stack_ptr = self.update_stack_pointer(1, stack_ptr)?;
                }
                Operation::LocalStore { variable_offset } => {
                    let value = self.stack_get(-1, stack_ptr)?;
                    let offset = calc_stack_offset(variable_offset);
                    self.stack_put(offset - 1, stack_ptr, value.into_int_value())?;
                    stack_ptr = self.update_stack_pointer(-1, stack_ptr)?;
                }

                // Arithmetic
                Operation::Neg => self.unary_op(stack_ptr, Builder::build_int_neg, "neg")?,
                Operation::Not => self.unary_op(stack_ptr, Builder::build_not, "not")?,

                Operation::Add => self.binary_op(&mut stack_ptr, Builder::build_int_add, "add")?,
                Operation::Sub => self.binary_op(&mut stack_ptr, Builder::build_int_sub, "sub")?,
                Operation::Mul => self.binary_op(&mut stack_ptr, Builder::build_int_mul, "mul")?,
                Operation::Div => {
                    self.binary_op(&mut stack_ptr, Builder::build_int_signed_div, "div")?
                }
                Operation::Mod => {
                    self.binary_op(&mut stack_ptr, Builder::build_int_signed_rem, "mod")?
                }
                Operation::And => self.binary_op(&mut stack_ptr, Builder::build_and, "and")?,
                Operation::Or => self.binary_op(&mut stack_ptr, Builder::build_or, "or")?,
                Operation::Xor => self.binary_op(&mut stack_ptr, Builder::build_xor, "xor")?,
                Operation::Lsh => {
                    self.binary_op(&mut stack_ptr, Builder::build_left_shift, "lsh")?
                }
                Operation::Rsh => self.binary_op(
                    &mut stack_ptr,
                    |b: &Builder, lhs, rhs, name| b.build_right_shift(lhs, rhs, true, name), // TODO true, false? shall we need to add different rsh like in Java?
                    "rsh",
                )?,

                // Comparison
                Operation::Equal => {
                    self.comparison(&mut vstack, IntPredicate::EQ);
                }
                Operation::NotEqual => {
                    self.comparison(&mut vstack, IntPredicate::NE);
                }
                Operation::LessThan => {
                    self.comparison(&mut vstack, IntPredicate::SLT); // TODO: signed ???
                }
                Operation::GreaterThan => {
                    self.comparison(&mut vstack, IntPredicate::SGT); // TODO: signed ???
                }
                Operation::LessThanOrEqual => {
                    self.comparison(&mut vstack, IntPredicate::SLE); // TODO: signed ???
                }
                Operation::GreaterThanOrEqual => {
                    self.comparison(&mut vstack, IntPredicate::SGE); // TODO: signed ???
                }

                // TODO: maybe break on non-supported operations
                _ => {
                    return None;
                }
            }
        }

        vstack.finalize();

        Some(BasicValueEnum::PointerValue(
            self.update_stack_pointer(vstack.get_offset(), stack_ptr)?,
        ))
    }

    fn stack_put(
        &self,
        offset: i64,
        stack_ptr: PointerValue<'ctx>,
        value: IntValue<'_>,
    ) -> Option<()> /* TODO: result */ {
        self.builder
            .build_store(
                self.ptr_with_offset(offset, "store_stack_ptr", stack_ptr)?,
                value,
            )
            .ok()?;
        Some(())
    }

    fn stack_get(
        &self,
        offset: i64,
        stack_ptr: PointerValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> /* TODO: result */ {
        self.builder
            .build_load(
                self.context.i64_type(),
                self.ptr_with_offset(offset, "load_stack_ptr", stack_ptr)?,
                "value_on_stack",
            )
            .ok()
    }

    fn update_stack_pointer(
        &self,
        offset: i64,
        stack_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        Some(self.ptr_with_offset(offset, "new_stack_ptr", stack_ptr)?)
    }

    fn ptr_with_offset(
        &self,
        offset: i64,
        name: &str,
        current_ptr: PointerValue<'ctx>,
    ) -> Option<PointerValue<'ctx>> /* TODO: result */ {
        ptr_with_offset(
            offset,
            name,
            current_ptr,
            self.context.i64_type(),
            &self.builder,
        )
    }

    fn comparison<'s>(&'s self, vstack: &mut VirtualStack<'s, 's>, op: IntPredicate) {
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

    fn unary_op<F>(&self, stack_ptr: PointerValue<'ctx>, op: F, unary_str: &str) -> Option<()>
    where
        F: FnOnce(&Builder<'ctx>, IntValue<'ctx>, &str) -> Result<IntValue<'ctx>, BuilderError>,
    {
        let val = self.stack_get(-1, stack_ptr)?;
        assert!(val.is_int_value());

        let unary_op = op(&self.builder, val.into_int_value(), unary_str).ok()?;
        self.stack_put(-1, stack_ptr, unary_op)?;

        Some(())
    }

    fn binary_op<F>(
        &self,
        stack_ptr: &mut PointerValue<'ctx>,
        op: F,
        binary_str: &str,
    ) -> Option<()>
    where
        F: FnOnce(
            &Builder<'ctx>,
            IntValue<'ctx>,
            IntValue<'ctx>,
            &str,
        ) -> Result<IntValue<'ctx>, BuilderError>,
    {
        let rhs = self.stack_get(-1, *stack_ptr)?;
        let lhs = self.stack_get(-2, *stack_ptr)?;
        assert!(lhs.is_int_value()); // TODO return err
        assert!(rhs.is_int_value()); // TODO return err

        let binary_op = op(
            &self.builder,
            lhs.into_int_value(),
            rhs.into_int_value(),
            binary_str,
        )
        .ok()?;
        self.stack_put(-2, *stack_ptr, binary_op)?;
        *stack_ptr = self.update_stack_pointer(-1, *stack_ptr)?;

        Some(())
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
) -> Option<PointerValue<'ctx>> /* TODO: result */ {
    let offset_const = if offset < 0 {
        i64_type.const_int((-offset) as u64, true).const_neg()
    } else {
        i64_type.const_int(offset as u64, false)
    };
    unsafe { builder.build_in_bounds_gep(i64_type, current_ptr, &[offset_const], name) }.ok()
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
    fn test_jit_binary_arithmetic() {
        let context = Context::create();
        let codegen = init(&context);

        let mut stack: [TrombValue; 16] = [TrombValue::default(); 16];
        let stack_base = stack.as_mut_ptr();

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
                .jit_compile_basic_block(idx, &[op.clone()])
                .expect(&format!("Failed to compile {}", op_str));

            let new_stack_ptr = unsafe { jitted.call(offset_ptr(stack_base, 2)) };
            assert_eq!(stack[0], f(lhs, rhs), "{}", op_str);
            assert_eq!(new_stack_ptr, offset_ptr(stack_base, 1));
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

    fn offset_ptr(ptr: *mut u64, offset: i64) -> *mut u64 {
        (ptr as i64 + offset * std::mem::size_of::<u64>() as i64) as *mut u64
    }
}
