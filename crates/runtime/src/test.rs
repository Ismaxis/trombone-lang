#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use inkwell::OptimizationLevel;
    use inkwell::context::Context;

    use rand::{Rng, SeedableRng};
    use rand_pcg::Pcg64;
    use trombone_common::TrombValue;
    use trombone_common::bytecode::{Instruction, Operation};
    use trombone_jit::{CodeGen, CodeGenTrait};
    use trombone_runner::runner::{ArrayOperationStream, Runner};

    type CurrentStackSize = usize;
    type StackDiff = isize;

    type StackDifference = dyn Fn(CurrentStackSize) -> Option<StackDiff>; // if operation is permitted, returns how many stack grows after operation
    type Generator = dyn Fn(CurrentStackSize) -> Operation;

    /**
     * Generate specific instruction in context of current stack machine
     */
    struct OperationGenerator {
        pred: Box<StackDifference>, // stack pointer goes after operation
        gener: Box<Generator>,      // generator of operation
    }

    impl OperationGenerator {
        fn new(pred: Box<StackDifference>, gener: Box<Generator>) -> Self {
            Self { pred, gener }
        }
    }

    impl From<(Box<StackDifference>, Box<Generator>)> for OperationGenerator {
        fn from(value: (Box<StackDifference>, Box<Generator>)) -> Self {
            Self::new(value.0, value.1)
        }
    }

    fn constant_op<T: 'static + Copy>(val: T) -> Box<dyn Fn(CurrentStackSize) -> T> {
        Box::new(move |_| val)
    }

    fn depth_pred(need_operands: CurrentStackSize, diff: StackDiff) -> Box<StackDifference> {
        Box::new(move |cur_stack_size| (cur_stack_size >= need_operands).then_some(diff))
    }

    fn random_value<T: Fn(i32) -> Operation + 'static>(
        val_creator: T,
        gener: &Rc<RefCell<Pcg64>>,
    ) -> Box<Generator> {
        let new_gener = gener.clone();
        Box::new(move |_| val_creator(new_gener.borrow_mut().random::<i32>()))
    }

    /**
     * Create Operation with offset in range (0..rsp-2)
     */
    fn within_stack<T: Fn(i32) -> Operation + 'static>(
        val_creator: T,
        gener: &Rc<RefCell<Pcg64>>,
    ) -> Box<Generator> {
        // TODO: same code with within_stack_without_last, need generalization
        let new_gener = gener.clone();
        Box::new(move |cur_depth| {
            val_creator(
                new_gener
                    .borrow_mut()
                    .random_range(0..cur_depth)
                    .try_into()
                    .unwrap(),
            )
        })
    }

    /**
     * Create Operation with offset in range (0..rsp-2)
     */
    fn within_stack_without_last<T: Fn(i32) -> Operation + 'static>(
        val_creator: T,
        gener: &Rc<RefCell<Pcg64>>,
    ) -> Box<Generator> {
        // TODO: same code with within_stack, need generalization
        let new_gener = gener.clone();
        Box::new(move |cur_depth| {
            val_creator(
                new_gener
                    .borrow_mut()
                    .random_range(0..cur_depth - 1)
                    .try_into()
                    .unwrap(),
            )
        })
    }

    fn init(context: &Context) -> (CodeGen, Rc<RefCell<Pcg64>>, Vec<OperationGenerator>) // TODO better result
    {
        let generator = Rc::new(RefCell::new(Pcg64::seed_from_u64(SEED)));

        #[rustfmt::skip]
        let operations: Vec<OperationGenerator> = [
            // stack
            (depth_pred(0, 1),     random_value(|value|                        Operation::PushLiteral { value },          &generator)).into(),
            (depth_pred(1, -1),    constant_op(                                Operation::Pop                                       )).into(),
            (depth_pred(1, 1),     within_stack(|variable_offset|              Operation::LocalCopy { variable_offset },  &generator)).into(),
            (depth_pred(2, -1),    within_stack_without_last(|variable_offset| Operation::LocalStore { variable_offset }, &generator)).into(),

            // arithmetic
            (depth_pred(1, 0),  constant_op(Operation::Neg)).into(),
            (depth_pred(1, 0),  constant_op(Operation::Not)).into(), 
            (depth_pred(2, -1), constant_op(Operation::Add)).into(),
            (depth_pred(2, -1), constant_op(Operation::Sub)).into(),
            (depth_pred(2, -1), constant_op(Operation::Mul)).into(),
            (depth_pred(2, -1), constant_op(Operation::Div)).into(),
            (depth_pred(2, -1), constant_op(Operation::Mod)).into(),
            (depth_pred(2, -1), constant_op(Operation::And)).into(),
            (depth_pred(2, -1), constant_op(Operation::Or)).into(),
            (depth_pred(2, -1), constant_op(Operation::Xor)).into(),
            // Lsh and Rsh are disabled due to unconsistency of shifting at negative count.
            // (depth_pred(2, -1), constant_op(Operation::Lsh)).into(),   
            // (depth_pred(2, -1), constant_op(Operation::Rsh)).into(),
            
            // comparison
            (depth_pred(2, -1), constant_op(Operation::Equal)).into(),
            (depth_pred(2, -1), constant_op(Operation::NotEqual)).into(),
            (depth_pred(2, -1), constant_op(Operation::LessThan)).into(),
            (depth_pred(2, -1), constant_op(Operation::GreaterThan)).into(),
            (depth_pred(2, -1), constant_op(Operation::LessThanOrEqual)).into(),
            (depth_pred(2, -1), constant_op(Operation::GreaterThanOrEqual)).into(),
        ].into();

        // Jump instructions are forbidden to be JITted in basic block by design.
        // In the future, it will be possible to check whether all transitions remain closed.
        // P.S.: A transition is considered closed if its location is under compilation too.

        let module = context.create_module("coherence_test");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .expect("Failed to create JIT execution engine");

        let codegen = CodeGen::new(context, module, builder, execution_engine);

        (codegen, generator, operations)
    }

    const SEED: u64 = 14881337420;
    const DEPTH: usize = 15;
    const ITERATIONS: usize = 4000;

    // If the program runs without errors in the interpreter, then it should run identically in the JITted version
    #[test]
    fn test_coherence_test() -> trombone_common::error::Result<()> {
        let context = Context::create();
        let (codegen, generator, operations) = init(&context);

        for iter in 0..ITERATIONS {
            // TODO: need more flexible test infrastructure
            let mut operations_as_u64 = [0; DEPTH];
            let mut operations_as_operations = Vec::with_capacity(DEPTH);

            let mut stack_size: usize = 0;
            for item in operations_as_u64.iter_mut().take(DEPTH) {
                let possible_operations: Vec<&OperationGenerator> = operations
                    .iter()
                    .filter(|x| x.pred.as_ref()(stack_size).is_some())
                    .collect();
                let picked_instruction = possible_operations[generator
                    .borrow_mut()
                    .random_range(0..possible_operations.len())];
                let (stack_diff, operation) = (
                    picked_instruction.pred.as_ref()(stack_size).unwrap(),
                    picked_instruction.gener.as_ref()(stack_size),
                );
                operations_as_operations.push(operation);
                *item = Into::<Instruction>::into(operation).as_u64();

                stack_size = (stack_size as isize + stack_diff) as usize;
            }

            let op_stream = ArrayOperationStream::new(&operations_as_u64);
            let mut runner = Runner::new_with_defaults(op_stream);
            let mut errorneous = false;
            for _ in 0..DEPTH {
                let res = runner.evaluate_next_instruction();
                if res.is_err() {
                    errorneous = true;
                    break;
                }
            }

            let jitted = codegen
                .jit_compile_basic_block(iter, &operations_as_operations)
                .expect("JIT failed");
            let mut jit_stack: [TrombValue; 1024] = [0; 1024];
            let jit_stack_base = jit_stack.as_mut_ptr();

            let jit_new_stack_ptr = unsafe { jitted.call(jit_stack_base) };
            let jit_sp = unsafe { jit_new_stack_ptr.offset_from(jit_stack_base) };

            if errorneous {
                assert!(
                    jit_new_stack_ptr.is_null(),
                    "Jit is not errorneous, but should. iter = {}, ops = {:?}",
                    iter,
                    operations_as_operations
                );
            } else {
                assert_eq!(
                    runner.sp,
                    jit_sp.try_into().unwrap(),
                    "Pointers are not same, iter = {}, ops = {:?}",
                    iter,
                    operations_as_operations
                );
                assert_eq!(
                    runner.stack[0..runner.sp],
                    jit_stack[0..runner.sp],
                    "Stacks are not same, iter = {}, ops = {:?}",
                    iter,
                    operations_as_operations
                );
            }
        }

        Ok(())
    }
}
