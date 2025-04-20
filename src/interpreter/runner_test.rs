#[cfg(test)]
mod tests {
    use crate::bytecode::Instruction;
    use crate::error::Result;
    use crate::interpreter::runner::{OperationStream, Runner};
    use crate::opcode;

    #[test]
    fn test_basic_arithmetic() -> Result<()> {
        // 1 + 2 = 3
        let mut instructions = [0u64; 1024];
        instructions[0] = Instruction::from_parts(0x0002, 0x00000001).as_u64(); // STORE 1
        instructions[1] = Instruction::from_parts(0x0002, 0x00000002).as_u64(); // STORE 2
        instructions[2] = Instruction::from_parts(0x0001, 0x0BADF00D).as_u64(); // ADD

        let stream = OperationStream::new(instructions);
        let mut runner = Runner::new(stream);

        runner.evaluate_next_instruction()?;
        runner.evaluate_next_instruction()?;
        runner.evaluate_next_instruction()?;

        assert_eq!(runner.sp, 1);
        assert_eq!(runner.stack[runner.sp - 1], 3);

        Ok(())
    }

    #[test]
    fn test_jump_operations() -> Result<()> {
        // Test for jump operations
        let mut instructions = [0u64; 1024];

        // Setup: Store values and then test jumps
        instructions[0] = Instruction::from_parts(0x0002, 0x00000001).as_u64(); // STORE 1
        instructions[1] = Instruction::from_parts(0x0002, 0x00000000).as_u64(); // STORE 0

        // Test unconditional jump (OP_JMP)
        // Jump to instruction 5, skipping the next instruction
        instructions[2] = Instruction::from_parts(opcode::OP_JMP, 5 - 2).as_u64();

        // This should be skipped
        instructions[3] = 0x0;
        instructions[4] = 0x0;

        // Test conditional jump (OP_JMP_IF)
        // Store 5, then jump to 9 if top of stack is non-zero
        instructions[5] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[6] = Instruction::from_parts(opcode::OP_JMP_IF, 9 - 6).as_u64();

        // This should be skipped
        instructions[7] = 0x0;
        instructions[8] = 0x0;

        // Test conditional jump (OP_JMP_IF_NOT)
        // Store 0, then jump to 13 if top of stack is zero
        instructions[9] = Instruction::from_parts(0x0002, 0x00000000).as_u64(); // STORE 0
        instructions[10] = Instruction::from_parts(opcode::OP_JMP_IF_NOT, 13 - 10).as_u64();

        // This should be skipped
        instructions[11] = 0x0;
        instructions[12] = 0x0;

        // Final value to confirm we reached the end
        instructions[13] = Instruction::from_parts(0x0002, 0x0000000A).as_u64(); // STORE 10

        let stream = OperationStream::new(instructions);
        let mut runner = Runner::new(stream);

        for _ in 0..8 {
            runner.evaluate_next_instruction()?;
        }

        // Check final stack state
        assert_eq!(runner.sp, 3);
        assert_eq!(runner.stack[0], 1);
        assert_eq!(runner.stack[1], 0);
        assert_eq!(runner.stack[2], 10);

        assert_eq!(runner.stream.instruction_pointer, 14);

        Ok(())
    }

    #[test]
    fn test_comparison_operations() -> Result<()> {
        // Test for comparison operations
        let mut instructions = [0u64; 1024];

        // Test equality (5 == 5)
        instructions[0] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[1] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[2] = Instruction::from_parts(opcode::OP_EQ, 0).as_u64(); // ==

        // Test inequality (5 != 3)
        instructions[3] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[4] = Instruction::from_parts(0x0002, 0x00000003).as_u64(); // STORE 3
        instructions[5] = Instruction::from_parts(opcode::OP_NE, 0).as_u64(); // !=

        // Test less than (3 < 5)
        instructions[6] = Instruction::from_parts(0x0002, 0x00000003).as_u64(); // STORE 3
        instructions[7] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[8] = Instruction::from_parts(opcode::OP_LT, 0).as_u64(); // <

        // Test greater than (5 > 3)
        instructions[9] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[10] = Instruction::from_parts(0x0002, 0x00000003).as_u64(); // STORE 3
        instructions[11] = Instruction::from_parts(opcode::OP_GT, 0).as_u64(); // >

        // Test less than or equal (3 <= 3)
        instructions[12] = Instruction::from_parts(0x0002, 0x00000003).as_u64(); // STORE 3
        instructions[13] = Instruction::from_parts(0x0002, 0x00000003).as_u64(); // STORE 3
        instructions[14] = Instruction::from_parts(opcode::OP_LE, 0).as_u64(); // <=

        // Test greater than or equal (5 >= 3)
        instructions[15] = Instruction::from_parts(0x0002, 0x00000005).as_u64(); // STORE 5
        instructions[16] = Instruction::from_parts(0x0002, 0x00000003).as_u64(); // STORE 3
        instructions[17] = Instruction::from_parts(opcode::OP_GE, 0).as_u64(); // >=

        let stream = OperationStream::new(instructions);
        let mut runner = Runner::new(stream);

        for _ in 0..18 {
            runner.evaluate_next_instruction()?;
        }

        // Check results - stack should have 6 boolean values (1s for true)
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
}