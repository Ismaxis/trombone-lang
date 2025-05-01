#[cfg(test)]
mod test {
    use core::panic;

    use trombone_common::{bytecode::Operation, error::Result, TrombValue};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use trombone_runner::runner::{ArrayOperationStream, OperationStream};

    static RANDOM_SEED: u64 = 12345678987u64;

    fn generate_operation_stream<'a>(generator: StdRng, depth: usize) -> ArrayOperationStream<'a> {
        let mut instructions = [TrombValue; 1024];
        instructions.reserve_exact(depth);

        for _ in 0..depth {
            use trombone_common::bytecode::Operation::*;
            instructions.push(match generator.random::<u64>() % 22 {
                0 => PushLiteral {
                    value: generator.random_range(0..=100),
                },
                1 => Pop,
                2 => LocalCopy {
                    variable_offset: generator.random_range(0..=3),
                },
                3 => LocalStore {
                    variable_offset: generator.random_range(0..=3),
                },
                4 => Neg,
                5 => Not,
                6 => Add,
                7 => Sub,
                8 => Mul,
                9 => Div,
                10 => Mod,
                11 => And,
                12 => Or,
                13 => Xor,
                14 => Lsh,
                15 => Rsh,
                16 => Equal,
                17 => NotEqual,
                18 => LessThan,
                19 => GreaterThan,
                20 => LessThanOrEqual,
                21 => GreaterThanOrEqual,
                _ => panic!("Invalid operation"),
            });
        }

        ArrayOperationStream::new(&instructions)
    }

    #[test]
    fn interpreter_equal_jit() -> Result<()> {
        let generator = StdRng::seed_from_u64(RANDOM_SEED);
        let op_stream = generate_operation_stream(generator, 10);

        let runner = trombone_runner::runner::Runner::new(op_stream);

        Ok(())
    }
}