#[cfg(test)]
mod tests {
    use std::vec;

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
    }

    #[test]
    fn test_stack_operations() -> Result<()> {
        let mut runner = Runner::new(TestOperationStream::new());

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
        let mut runner = Runner::new(TestOperationStream::new());

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
            (Operation::Neg, opcode::OP_NEG, Box::new(|x| -(x as i64) as TrombValue)),
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

            let mut runner = Runner::new(stream);

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

            let mut runner = Runner::new(stream);

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
            let mut runner = Runner::new(stream);

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
        let mut runner = Runner::new(stream);

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
        let mut runner = Runner::new(stream);

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

    #[test]
    fn test_heap_operations() -> Result<()> {
        use opcode::*;
        const IGNORED: Immediate = 0x0;
        let mut instructions = [0u64; 1024];
        let mut stack = [0u64; 1024];

        stack[0] = 1; // Bytes to allocate
        instructions[0] = Instruction::from_parts(OP_HEAP_ALLOC, IGNORED).as_u64();
        instructions[1] = Instruction::from_parts(OP_HEAP_POP_PTR, IGNORED).as_u64();

        let stream = ArrayOperationStream::new(&instructions);
        let mock_allocator = MockAllocator {
            alloc_count: AtomicUsize::new(0),
        };
        let mut runner = Runner::new_with_allocator(stream, &mock_allocator);

        for v in stack.iter().take(1) {
            runner.stack[runner.sp] = *v as i64;
            runner.sp += 1;
        }

        // Test for heap operations
        runner.evaluate_next_instruction()?;
        assert_eq!(runner.sp, 1);
        assert_ne!(runner.stack[runner.sp - 1], 0);
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 1);
        assert_eq!(runner.stream.instruction_pointer, 1);

        runner.evaluate_next_instruction()?;
        assert_eq!(runner.sp, 0);
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);
        assert_eq!(runner.stream.instruction_pointer, 2);

        Ok(())
    }
}
