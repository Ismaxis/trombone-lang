#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::vec;

    use crate::control_block;
    use crate::runner::{ArrayOperationStream, OperationStream, Runner};
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use trombone_common::TrombValue;
    use trombone_common::bytecode::{Immediate, Instruction, Operation};
    use trombone_common::error::Result;
    use trombone_common::opcode::{self};

    struct MockAllocator {
        alloc_count: AtomicUsize,
        // TODO: track allocations and deallocations more precisely
    }

    unsafe impl GlobalAlloc for MockAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            self.alloc_count.fetch_add(1, Ordering::SeqCst);
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.alloc_count.fetch_sub(1, Ordering::SeqCst);
            unsafe { System.dealloc(ptr, layout) }
        }
    }

    struct TestOperationStream {
        instructions: std::vec::Vec<u64>,
        instruction_pointer: usize,
    }

    impl TestOperationStream {
        fn new() -> TestOperationStream {
            TestOperationStream {
                instructions: std::vec::Vec::new(),
                instruction_pointer: 0,
            }
        }

        fn emplace_instruction(&mut self, opcode: u8, immediate: Immediate) {
            self.instructions
                .push(Instruction::from_parts(opcode, immediate).as_u64());
        }

        fn emplace_instruction_raw(&mut self, raw: u64) {
            self.instructions.push(raw);
        }
    }

    impl OperationStream for TestOperationStream {
        fn next_instruction(&mut self) -> Result<Operation> {
            let res = self.instructions[self.instruction_pointer];
            self.instruction_pointer += 1;
            Instruction::from_u64(res).try_into()
        }

        fn switch_frame(&mut self, offset: i32) {
            self.instruction_pointer = ((self.instruction_pointer as i64) + offset as i64) as usize;
        }

        fn get_instruction_pointer(&self) -> usize {
            self.instruction_pointer
        }

        fn get_instructions_len(&self) -> usize {
            todo!()
        }

        fn get_next_n(&mut self, n: usize) -> Vec<Operation> {
            self.instructions[self.instruction_pointer..]
                .iter()
                .take(n)
                .map(|&x| Instruction::from_u64(x).try_into().unwrap())
                .collect()
        }
    }

    #[test]
    fn test_stack_operations() -> Result<()> {
        let mut runner = Runner::new_with_defaults(TestOperationStream::new());

        // let x = 42;
        runner.stream.emplace_instruction(opcode::OP_PUSH, 42);
        runner.evaluate_next_instruction()?;
        // let y = 54;
        runner.stream.emplace_instruction(opcode::OP_PUSH, 54);
        runner.evaluate_next_instruction()?;
        // let y = 68;
        runner.stream.emplace_instruction(opcode::OP_PUSH, 68);
        runner.evaluate_next_instruction()?;

        // add five zeros
        for _ in 3..8 {
            runner.stream.emplace_instruction(opcode::OP_PUSH, 0);
            runner.evaluate_next_instruction()?;
        }

        // let x1 = x;
        // let y1 = y;
        // let z1 = z;
        for _ in 8..11 {
            runner.stream.emplace_instruction(opcode::OP_LOCAL_COPY, 7);
            runner.evaluate_next_instruction()?;
        }

        // checking correctness of stack
        let stack_should_be = vec![42, 54, 68, 0, 0, 0, 0, 0, 42, 54, 68];
        assert_eq!(stack_should_be, runner.stack[0..runner.sp]);

        // y1 = y1 + z1
        runner.stream.emplace_instruction(opcode::OP_ADD, 0);
        runner.evaluate_next_instruction()?;

        // x1 = x1 + y1
        runner.stream.emplace_instruction(opcode::OP_ADD, 0);
        runner.evaluate_next_instruction()?;

        // x = x1
        runner.stream.emplace_instruction(opcode::OP_LOCAL_STORE, 7);
        runner.evaluate_next_instruction()?;

        let stack_should_be = vec![42 + 54 + 68, 54, 68, 0, 0, 0, 0, 0];
        assert_eq!(stack_should_be, runner.stack[0..runner.sp]);

        Ok(())
    }

    #[test]
    fn test_corner_cases_stack_operations() -> Result<()> {
        let mut runner = Runner::new_with_defaults(TestOperationStream::new());

        // let x = 42;
        runner.stream.emplace_instruction(opcode::OP_PUSH, 42);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.stack[0..runner.sp], vec![42]);

        // OP_LOCAL_COPY {with offset zero} means "put top value on top"
        // let x1 = x;
        runner.stream.emplace_instruction(opcode::OP_LOCAL_COPY, 0);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.stack[0..runner.sp], vec![42, 42]);

        // ===========

        runner.stream.emplace_instruction(opcode::OP_PUSH, 1000);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.stack[0..runner.sp], vec![42, 42, 1000]);

        // OP_LOCAL_STORE {with offset zero} means "put top value on next after top value"
        // or, in some absurd sense "pop second value"
        runner.stream.emplace_instruction(opcode::OP_LOCAL_STORE, 0);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.stack[0..runner.sp], vec![42, 1000]);

        Ok(())
    }

    #[test]
    fn test_unary_arithmetic() -> Result<()> {
        #[rustfmt::skip]
        #[allow(clippy::type_complexity)]
        let tests: [(_, _, Box<dyn Fn(TrombValue) -> TrombValue>); 2] = [
            (Operation::Neg, opcode::OP_NEG, Box::new(|x| -{ x } as TrombValue)),
            (Operation::Not, opcode::OP_NOT, Box::new(|x| !x)),
        ];

        let operand = 42;
        for (op, opcode, res_fun) in tests {
            let mut instructions = [0u64; 1024];

            // Check if mapping from Opcode to Operation is correct
            let instruction = Instruction::from_parts(opcode, 0x0DEDBEEF);
            let operation: Operation = instruction.try_into()?;
            assert_eq!(operation, op);

            // Check result
            instructions[0] = Instruction::from_parts(opcode::OP_PUSH, operand).as_u64();
            instructions[1] = Instruction::from_parts(opcode, 0x0DEDBEEF).as_u64();

            let stream = ArrayOperationStream::new(&instructions);

            let mut runner = Runner::new_with_defaults(stream);

            runner.evaluate_next_instruction()?;
            runner.evaluate_next_instruction()?;

            assert_eq!(runner.sp, 1);
            assert_eq!(runner.stack[runner.sp - 1], res_fun(operand as TrombValue));
        }

        Ok(())
    }

    #[test]
    fn test_binary_arithmetic() -> Result<()> {
        #[rustfmt::skip]
        #[allow(clippy::type_complexity)]
        let tests: [(_, _, Box<dyn Fn(TrombValue, TrombValue) -> TrombValue>); 10]= [
            (Operation::Add,    opcode::OP_ADD, Box::new(|x, y| { x + y })),
            (Operation::Sub,    opcode::OP_SUB, Box::new(|x, y| { x - y })),
            (Operation::Mul,    opcode::OP_MUL, Box::new(|x, y| { x * y })),
            (Operation::Div,    opcode::OP_DIV, Box::new(|x, y| { x / y })), 
            (Operation::Mod,    opcode::OP_MOD, Box::new(|x, y| { x % y })),
            (Operation::And,    opcode::OP_AND, Box::new(|x, y| { x & y })),
            (Operation::Or,     opcode::OP_OR,  Box::new(|x, y| { x | y })),
            (Operation::Xor,    opcode::OP_XOR, Box::new(|x, y| { x ^ y })),
            (Operation::Lsh,    opcode::OP_LSH, Box::new(|x, y| { x << y })),
            (Operation::Rsh,    opcode::OP_RSH, Box::new(|x, y| { x >> y })),
        ];

        let op1 = 42;
        let op2 = 5;
        for (op, opcode, res_fun) in tests {
            let mut instructions = [0u64; 1024];

            // Check if mapping from Opcode to Operation is correct
            let instruction = Instruction::from_parts(opcode, 0x0DEDBEEF);
            let operation: Operation = instruction.try_into()?;
            assert_eq!(operation, op);

            // Check result
            instructions[0] = Instruction::from_parts(opcode::OP_PUSH, op1).as_u64();
            instructions[1] = Instruction::from_parts(opcode::OP_PUSH, op2).as_u64();
            instructions[2] = Instruction::from_parts(opcode, 0x0DEDBEEF).as_u64();

            let stream = ArrayOperationStream::new(&instructions);

            let mut runner = Runner::new_with_defaults(stream);

            runner.evaluate_next_instruction()?;
            runner.evaluate_next_instruction()?;
            runner.evaluate_next_instruction()?;

            assert_eq!(runner.sp, 1);
            assert_eq!(
                runner.stack[runner.sp - 1],
                res_fun(op1 as TrombValue, op2 as TrombValue)
            );
        }

        Ok(())
    }

    #[test]
    fn test_corner_case_arithmetic() -> Result<()> {
        for opcode in [opcode::OP_DIV, opcode::OP_MOD] {
            let mut instructions = [0u64; 1024];

            instructions[0] = Instruction::from_parts(opcode::OP_PUSH, 42).as_u64();
            instructions[1] = Instruction::from_parts(opcode::OP_PUSH, 0).as_u64();
            instructions[2] = Instruction::from_parts(opcode, 0x0DEDBEEF).as_u64();

            let stream = ArrayOperationStream::new(&instructions);
            let mut runner = Runner::new_with_defaults(stream);

            runner.evaluate_next_instruction()?;
            runner.evaluate_next_instruction()?;

            assert!(
                runner.evaluate_next_instruction().is_err(),
                "expected error due to division by zero"
            );
        }

        Ok(())
    }

    #[test]
    fn test_jump_operations() -> Result<()> {
        // Test for jump operations
        let mut instructions = [0u64; 1024];

        instructions[0] = Instruction::from_parts(opcode::OP_PUSH, 0x00000001).as_u64();
        instructions[1] = Instruction::from_parts(opcode::OP_PUSH, 0x00000000).as_u64();

        // Test unconditional jump (OP_JMP)
        instructions[2] = Instruction::from_parts(opcode::OP_JMP, 5 - 2).as_u64();

        // SKIP for the first time
        instructions[3] = Instruction::from_parts(opcode::OP_PUSH, 0x0000000A).as_u64();
        instructions[4] = 0x0;

        // Test conditional jump (OP_JMP_IF)
        instructions[5] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64();
        instructions[6] = Instruction::from_parts(opcode::OP_JMP_IF, 9 - 6).as_u64();

        // SKIP for the first time
        // Test conditional jump back (OP_JMP_IF)
        instructions[7] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64();
        instructions[8] = Instruction::from_parts(opcode::OP_JMP_IF, 3 - 8).as_u64();

        // Test conditional jump (OP_JMP_IF_NOT)
        instructions[9] = Instruction::from_parts(opcode::OP_PUSH, 0x00000000).as_u64();
        instructions[10] = Instruction::from_parts(opcode::OP_JMP_IF_NOT, 13 - 10).as_u64();

        // SKIP for the first time
        // Test conditional jump back (OP_JMP_IF_NOT)
        instructions[11] = Instruction::from_parts(opcode::OP_PUSH, 0x00000000).as_u64();
        instructions[12] = Instruction::from_parts(opcode::OP_JMP_IF_NOT, 7 - 12).as_u64();

        // Test unconditional jump back (OP_JMP)
        instructions[13] = Instruction::from_parts(opcode::OP_JMP, 11 - 13).as_u64();

        let stream = ArrayOperationStream::new(&instructions);
        let mut runner = Runner::new_with_defaults(stream);

        for _ in 0..13 {
            runner.evaluate_next_instruction()?;
        }

        assert_eq!(runner.sp, 3);
        assert_eq!(runner.stack[0], 1);
        assert_eq!(runner.stack[1], 0);
        assert_eq!(runner.stack[2], 10);

        assert_eq!(runner.stream.instruction_pointer, 4);

        Ok(())
    }

    #[test]
    fn test_comparison_operations() -> Result<()> {
        // Test for comparison operations
        let mut instructions = [0u64; 1024];

        // Test equality (5 == 5)
        instructions[0] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64(); // STORE 5
        instructions[1] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64(); // STORE 5
        instructions[2] = Instruction::from_parts(opcode::OP_EQ, 0).as_u64(); // ==

        // Test inequality (5 != 3)
        instructions[3] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64(); // STORE 5
        instructions[4] = Instruction::from_parts(opcode::OP_PUSH, 0x00000003).as_u64(); // STORE 3
        instructions[5] = Instruction::from_parts(opcode::OP_NE, 0).as_u64(); // !=

        // Test less than (3 < 5)
        instructions[6] = Instruction::from_parts(opcode::OP_PUSH, 0x00000003).as_u64(); // STORE 3
        instructions[7] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64(); // STORE 5
        instructions[8] = Instruction::from_parts(opcode::OP_LT, 0).as_u64(); // <

        // Test greater than (5 > 3)
        instructions[9] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64(); // STORE 5
        instructions[10] = Instruction::from_parts(opcode::OP_PUSH, 0x00000003).as_u64(); // STORE 3
        instructions[11] = Instruction::from_parts(opcode::OP_GT, 0).as_u64(); // >

        // Test less than or equal (3 <= 3)
        instructions[12] = Instruction::from_parts(opcode::OP_PUSH, 0x00000003).as_u64(); // STORE 3
        instructions[13] = Instruction::from_parts(opcode::OP_PUSH, 0x00000003).as_u64(); // STORE 3
        instructions[14] = Instruction::from_parts(opcode::OP_LE, 0).as_u64(); // <=

        // Test greater than or equal (5 >= 3)
        instructions[15] = Instruction::from_parts(opcode::OP_PUSH, 0x00000005).as_u64(); // STORE 5
        instructions[16] = Instruction::from_parts(opcode::OP_PUSH, 0x00000003).as_u64(); // STORE 3
        instructions[17] = Instruction::from_parts(opcode::OP_GE, 0).as_u64(); // >=

        let stream = ArrayOperationStream::new(&instructions);
        let mut runner = Runner::new_with_defaults(stream);

        for _ in 0..18 {
            runner.evaluate_next_instruction()?;
        }

        assert_eq!(runner.sp, 6);
        assert_eq!(runner.stack[0], 1); // 5 == 5 -> true
        assert_eq!(runner.stack[1], 1); // 5 != 3 -> true
        assert_eq!(runner.stack[2], 1); // 3 < 5 -> true
        assert_eq!(runner.stack[3], 1); // 5 > 3 -> true
        assert_eq!(runner.stack[4], 1); // 3 <= 3 -> true
        assert_eq!(runner.stack[5], 1); // 5 >= 3 -> true

        assert_eq!(runner.stream.instruction_pointer, 18);

        Ok(())
    }

    type RunnerType<'a, 'ctx> =
        Runner<'a, 'ctx, TestOperationStream, std::io::BufReader<std::io::Stdin>, std::io::Stdout>;

    #[test]
    fn test_heap_operations() -> Result<()> {
        use opcode::*;
        const IGNORED: Immediate = 0x0;

        let mock_allocator = MockAllocator {
            alloc_count: AtomicUsize::new(0),
        };

        let mut runner = Runner::new(
            TestOperationStream::new(),
            RunnerType::default_input(),
            RunnerType::default_output(),
            &mock_allocator,
        );

        let get_ref_count = |raw_ptr: i64| {
            let control_block =
                control_block::ControlBlock::from_value_ptr(raw_ptr as *const TrombValue);
            unsafe { (*control_block).ref_count() }
        };

        let load_values_into_stack =
            |base: usize, values: &[TrombValue], runner: &mut RunnerType| {
                for (i, x) in values.iter().enumerate() {
                    runner.stack[base + i] = *x;
                    runner.sp += 1;
                }
            };

        // ==== Test alloc and dealloc ====
        // alloc 1 TromValue
        load_values_into_stack(0, &[1], &mut runner);
        runner.stream.emplace_instruction(OP_HEAP_ALLOC, IGNORED);
        runner.evaluate_next_instruction()?;
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 1);
        assert_eq!(runner.sp, 1);
        assert_eq!(runner.stream.instruction_pointer, 1);
        assert_ne!(runner.stack[runner.sp - 1], 0);
        assert_eq!(get_ref_count(runner.stack[runner.sp - 1]), 0); // ref_count should be 0 after allocation

        // pop and dealloc
        runner.stream.emplace_instruction(OP_HEAP_POP_PTR, IGNORED);
        runner.evaluate_next_instruction()?;
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);
        assert_eq!(runner.sp, 0);
        assert_eq!(runner.stream.instruction_pointer, 2);

        // ==== Test copy ====
        // alloc x
        load_values_into_stack(0, &[1], &mut runner);
        runner.stream.emplace_instruction(OP_HEAP_ALLOC, IGNORED);

        // y = copy(x)
        runner.stream.emplace_instruction(OP_HEAP_COPY_PTR, 0x0);
        for _ in 0..2 {
            runner.evaluate_next_instruction()?;
        }
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 1);
        assert_eq!(runner.sp, 2);
        assert_eq!(runner.stream.instruction_pointer, 4);
        assert_ne!(runner.stack[runner.sp - 1], 0);
        assert_eq!(runner.stack[runner.sp - 1], runner.stack[runner.sp - 2]);
        assert_eq!(get_ref_count(runner.stack[runner.sp - 1]), 1); // ref_count should be 1: one for the copy

        // drop (y)
        runner.stream.emplace_instruction(OP_HEAP_POP_PTR, IGNORED);
        runner.evaluate_next_instruction()?;
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 1);
        assert_eq!(runner.sp, 1);
        assert_eq!(runner.stream.instruction_pointer, 5);
        assert_ne!(runner.stack[runner.sp - 1], 0);
        assert_eq!(get_ref_count(runner.stack[runner.sp - 1]), 0);

        // drop (x)
        runner.stream.emplace_instruction(OP_HEAP_POP_PTR, IGNORED);
        runner.evaluate_next_instruction()?;
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);
        assert_eq!(runner.sp, 0);
        assert_eq!(runner.stream.instruction_pointer, 6);

        // ==== Test load and store ====

        // alloc x
        load_values_into_stack(0, &[4], &mut runner);
        runner.stream.emplace_instruction(OP_HEAP_ALLOC, IGNORED);
        runner.evaluate_next_instruction()?;
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 1);
        assert_eq!(runner.sp, 1);

        let values = [
            /* store 72 at offset 3 */ 72, 3, /* store 62 at offset 2 */ 62, 2,
            /* store 52 at offset 1 */ 52, 1, /* store 42 at offset 0 */ 42, 0,
        ];
        load_values_into_stack(1, &values, &mut runner);
        for i in 0..4 {
            // [x + i] = (42 + i*10)
            runner
                .stream
                .emplace_instruction(OP_HEAP_STORE_PTR, 0x2 * (4 - i));
        }
        for _ in 0..4 {
            runner.evaluate_next_instruction()?;
        }
        assert_eq!(runner.sp, 1);
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 1);
        assert_eq!(runner.stream.instruction_pointer, 11);
        assert_ne!(runner.stack[runner.sp - 1], 0);
        assert_eq!(get_ref_count(runner.stack[runner.sp - 1]), 0);
        let ptr = runner.stack[runner.sp - 1] as *mut TrombValue;
        for i in 0..4 {
            assert_eq!(unsafe { *ptr.add(i) }, (42 + i * 10) as TrombValue);
        }

        // load x[i]
        for i in 0..4 {
            load_values_into_stack(1 + i, &[i as TrombValue], &mut runner);
            runner
                .stream
                .emplace_instruction(OP_HEAP_LOAD_PTR, 0x1 + i as i32);
            runner.evaluate_next_instruction()?;
            assert_eq!(runner.sp, 2 + i);
        }
        assert_eq!(runner.stack[1..5], vec![42, 52, 62, 72]);

        // drop (x)
        runner.sp = 1; // reset stack pointer to 1
        runner.stream.emplace_instruction(OP_HEAP_POP_PTR, IGNORED);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.sp, 0);

        // no leaks
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);
        Ok(())
    }

    #[test]
    fn test_io_instructions() -> Result<()> {
        let input = std::io::Cursor::new("42\n".as_bytes());
        let output = std::io::Cursor::new(Vec::new());

        let mut runner = Runner::new(
            TestOperationStream::new(),
            input,
            output,
            RunnerType::default_allocator(),
        );

        // read into x
        runner.stream.emplace_instruction(opcode::OP_READ, 0);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.sp, 1);
        assert_eq!(runner.stack[0], 42);

        // print(x);
        runner.stream.emplace_instruction(opcode::OP_PRINT, 0);
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.output.into_inner(), "> $$ 42\n".as_bytes());

        Ok(())
    }

    #[test]
    fn test_functions() -> Result<()> {
        {
            // Call with zero operand
            let mut runner = Runner::new_with_defaults(TestOperationStream::new());

            // foo();
            runner.stream.emplace_instruction(opcode::OP_PUSH, 0); // reserve space for return address
            runner
                .stream
                .emplace_instruction(opcode::OP_SET_RET_ADDRESS, 0); // zero operands
            runner.stream.emplace_instruction(opcode::OP_JMP, 5);

            for _ in 0..3 {
                runner.evaluate_next_instruction()?;
            }
            assert_eq!(runner.stack[0..runner.sp], [3]);
        }

        {
            // Call with one operand
            let mut runner = Runner::new_with_defaults(TestOperationStream::new());

            // foo(42);
            runner.stream.emplace_instruction(opcode::OP_PUSH, 0); // reserve space for return address
            runner.stream.emplace_instruction(opcode::OP_PUSH, 42); // push operand
            runner
                .stream
                .emplace_instruction(opcode::OP_SET_RET_ADDRESS, 1); // one operand
            runner.stream.emplace_instruction(opcode::OP_JMP, 5);

            for _ in 0..4 {
                runner.evaluate_next_instruction()?;
            }
            assert_eq!(runner.stack[0..runner.sp], [4, 42]);
        }

        {
            // Void return
            let mut runner = Runner::new_with_defaults(TestOperationStream::new());

            runner.stack[0] = 1488;
            runner.sp = 1;
            runner.stream.emplace_instruction(opcode::OP_RET, 0);

            runner.evaluate_next_instruction()?;
            assert_eq!(runner.sp, 0);
            assert_eq!(runner.stream.get_instruction_pointer(), 1488);
        }

        {
            // Result return
            let mut runner = Runner::new_with_defaults(TestOperationStream::new());

            runner.stack[0..2].copy_from_slice(&[0xDEADBEEF, 1337]);
            runner.sp = 2;
            runner.stream.emplace_instruction(opcode::OP_RET, 1);

            runner.evaluate_next_instruction()?;
            assert_eq!(runner.sp, 1);
            assert_eq!(&runner.stack[0..runner.sp], &[0xDEADBEEF]);
            assert_eq!(runner.stream.get_instruction_pointer(), 1337);
        }

        Ok(())
    }

    #[test]
    fn test_basic_block() -> Result<()> {
        let input = std::io::Cursor::new("42\n".as_bytes());
        let output = std::io::Cursor::new(Vec::new());

        let mock_allocator = MockAllocator {
            alloc_count: AtomicUsize::new(0),
        };

        type RunnerTypeLoc<'a> = Runner<
            'a,
            'a,
            TestOperationStream,
            std::io::Cursor<&'a [u8]>,
            std::io::Cursor<Vec<u8>>,
        >;

        let context = trombone_jit::ExportedContext::create();
        let codegen = trombone_jit::init(&context);
        let mut runner: RunnerTypeLoc =
            Runner::new(TestOperationStream::new(), input, output, &mock_allocator);

        const JIT_HIT_THRESHOLD: usize = 8;
        runner.set_codegen(codegen, JIT_HIT_THRESHOLD);

        let instructions = [
            0xf000000000000000, // READ
            0x0100000000000000, // PUSH 0
            0x0300000000000001, // LOCAL_COPY
            0xe000000000000000, // HEAP_ALLOC
            /* assign loop cond */
            0x0300000000000001, // LOCAL_COPY
            0x0300000000000003, // LOCAL_COPY
            0xc200000000000000, // LT
            0xd20000000000000a, // JMP_IF_NOT
            /* asssign loop body start */
            Instruction::from_parts(opcode::OP_BASICBLOCK_START, 0x7).as_u64(),
            0x0300000000000001, // LOCAL_COPY
            0x0300000000000002, // LOCAL_COPY
            0xe400000000000002, // HEAP_STORE_PTR
            0x0300000000000001, // LOCAL_COPY
            0x0100000000000001, // PUSH 1
            0xa200000000000000, // ADD
            0x0400000000000001, // LOCAL_STORE
            /* asssign loop body end */
            0xd0000000fffffff4, // JMP -12 (to loop cond)
            /* print loop */
            0x0100000000000000, // PUSH 0
            0x0400000000000001, // LOCAL_STORE
            0x0300000000000001, // LOCAL_COPY
            0x0300000000000003, // LOCAL_COPY
            0xc200000000000000, // LT
            0xd200000000000009, // JMP_IF_NOT
            0x0300000000000001, // LOCAL_COPY
            0xe300000000000001, // LOCAL_COPY
            0xf100000000000000, // PRINT
            0x0300000000000001, // LOCAL_COPY
            0x0100000000000001, // PUSH 1
            0xa200000000000000, // ADD
            0x0400000000000001, // LOCAL_STORE
            0xd0000000fffffff5, // JMP
            0xe100000000000000, // HEAP_POP_PTR
            0x0200000000000000, // POP
            0x0200000000000000, // POP
        ];

        for x in instructions {
            runner.stream.emplace_instruction_raw(x);
        }

        while runner.stream.get_instruction_pointer() < instructions.len() {
            match runner.evaluate_next_instruction() {
                Ok(crate::runner::ReturnCode::Continue) => {}
                Ok(crate::runner::ReturnCode::Done) => {
                    break;
                }
                Err(error) => {
                    println!(
                        "failed at instruction: {}",
                        runner.stream.get_instruction_pointer()
                    );
                    return Err(error);
                }
            }
        }

        assert_eq!(runner.sp, 0);
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);

        runner.output.flush()?;
        let output = String::from_utf8(runner.output.into_inner()).unwrap();
        assert_eq!(
            output,
            "> $$ 0\n$$ 1\n$$ 2\n$$ 3\n$$ 4\n$$ 5\n$$ 6\n$$ 7\n$$ 8\n$$ 9\n$$ 10\n$$ 11\n$$ 12\n$$ 13\n$$ 14\n$$ 15\n$$ 16\n$$ 17\n$$ 18\n$$ 19\n$$ 20\n$$ 21\n$$ 22\n$$ 23\n$$ 24\n$$ 25\n$$ 26\n$$ 27\n$$ 28\n$$ 29\n$$ 30\n$$ 31\n$$ 32\n$$ 33\n$$ 34\n$$ 35\n$$ 36\n$$ 37\n$$ 38\n$$ 39\n$$ 40\n$$ 41\n"
        );

        assert_eq!(
            runner.basic_block_stats,
            vec![(8, JIT_HIT_THRESHOLD)].into_iter().collect()
        );

        Ok(())
    }
}
