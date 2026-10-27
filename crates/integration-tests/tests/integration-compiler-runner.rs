#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use trombone_common::error::Result;

    use trombone_runner::runner::Runner;
    use trombone_runner::runner_test::test_utils::*;

    use trombone_compiler_lib::*;

    macro_rules! setup_runner {
        ($input:literal) => {
            Runner::new(
                SimpleTestOperationStream::new(),
                std::io::Cursor::new($input.as_bytes()),
                std::io::Cursor::new(Vec::new()),
                RunnerType::default_allocator(),
            )
        };
    }

    macro_rules! setup_runner_with_alloc {
        ($input:expr, $allocator:expr) => {{
            Runner::new(
                SimpleTestOperationStream::new(),
                std::io::Cursor::new($input.as_bytes()),
                std::io::Cursor::new(Vec::new()),
                $allocator,
            )
        }};
    }

    fn read_program(file_name: &str) -> Result<String> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/src")
            .join(file_name);
        Ok(std::fs::read_to_string(path)?)
    }

    fn run_test<'a>(
        runner: &mut Runner<
            'a,
            'a,
            SimpleTestOperationStream,
            std::io::Cursor<&'a [u8]>,
            std::io::Cursor<Vec<u8>>,
        >,
        program: &str,
    ) -> Result<String> {
        let ops = compiler::compile_from_string(program)?;
        runner.stream.emplace_operations(ops);
        runner.evaluate()?;

        Ok(String::from_utf8(runner.output.clone().into_inner()).unwrap())
    }

    #[test]
    fn no_return() -> Result<()> {
        let program = read_program("no_return.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn single_print() -> Result<()> {
        let program = read_program("single_print.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn read_and_print() -> Result<()> {
        let program = read_program("read_and_print.tromb")?;
        let mut runner = setup_runner!("42");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "> $ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn variables_access() -> Result<()> {
        let program = read_program("variables_access.tromb")?;
        let mut runner = setup_runner!("10\n100");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "> > $ 2783000000\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call() -> Result<()> {
        let program = read_program("function_call.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n$ 42\n$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_with_arguments() -> Result<()> {
        let program = read_program("function_call_with_arguments.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n$ 43\n$ 44\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_with_multiple_arguments() -> Result<()> {
        let program = read_program("function_call_with_multiple_arguments.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n$ 43\n$ 44\n$ 52\n$ 53\n$ 54\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_with_return() -> Result<()> {
        let program = read_program("function_call_with_return.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n$ 42\n$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_discard() -> Result<()> {
        let program = read_program("function_call_discard.tromb")?;
        let mut runner = setup_runner!("1\n1\n1\n");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "> > > ".to_string());
        Ok(())
    }

    #[test]
    fn function_call_inc() -> Result<()> {
        let program = read_program("function_call_inc.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 43\n$ 89\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_sum() -> Result<()> {
        let program = read_program("function_call_sum.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 3\n$ 7\n$ 10\n".to_string());
        Ok(())
    }

    // If

    #[test]
    fn basic_if() -> Result<()> {
        let program = read_program("basic_if.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn nested_if() -> Result<()> {
        let program = read_program("nested_if.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn if_return() -> Result<()> {
        let program = read_program("if_return.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n$ 0\n$ 0\n".to_string());
        Ok(())
    }

    #[test]
    fn if_return_hard() -> Result<()> {
        let program = read_program("if_return_hard.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 3\n$ 2\n$ 1\n$ 0\n".to_string());
        Ok(())
    }

    #[test]
    fn if_return_deep() -> Result<()> {
        let program = read_program("if_return_deep.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 2\n$ 10\n$ -1\n".to_string());
        Ok(())
    }

    #[test]
    fn if_clear_locals() -> Result<()> {
        let program = read_program("if_clear_locals.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 2\n$ 10\n$ -1\n".to_string());
        Ok(())
    }

    #[test]
    fn recursion() -> Result<()> {
        let program = read_program("recursion.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn basic_if_else() -> Result<()> {
        let program = read_program("basic_if_else.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n$ 2\n".to_string());
        Ok(())
    }

    #[test]
    fn if_chain() -> Result<()> {
        let program = read_program("if_chain.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n$ 2\n$ 3\n$ 4\n".to_string());
        Ok(())
    }

    #[test]
    fn if_chain_no_else() -> Result<()> {
        let program = read_program("if_chain_no_else.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n$ 3\n$ 2\n$ 3\n$ 3\n".to_string());
        Ok(())
    }

    #[test]
    fn simple_while() -> Result<()> {
        let program = read_program("simple_while.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(
            result,
            "$ 10\n$ 9\n$ 8\n$ 7\n$ 6\n$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string()
        );
        Ok(())
    }

    #[test]
    fn while_no_execution() -> Result<()> {
        let program = read_program("while_no_execution.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn inner_while() -> Result<()> {
        let program = read_program("inner_while.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(
            result,
            "$ 15\n$ 10\n$ 5\n$ 12\n$ 8\n$ 4\n$ 9\n$ 6\n$ 3\n$ 6\n$ 4\n$ 2\n$ 3\n$ 2\n$ 1\n"
                .to_string()
        );
        Ok(())
    }

    #[test]
    fn return_in_while() -> Result<()> {
        let program = read_program("return_in_while.tromb")?;
        let mut runner = setup_runner!("0");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "> $ 0\n".to_string());

        let mut runner = setup_runner!("123");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "> $ 1\n".to_string());

        Ok(())
    }

    #[test]
    fn return_nested_if_and_while() -> Result<()> {
        let program = read_program("return_nested_if_and_while.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "$ 1\n$ 0\n$ 1\n$ 0\n".to_string());

        Ok(())
    }

    #[test]
    fn while_condition_is_function_call() -> Result<()> {
        let program = read_program("while_condition_is_function_call.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn double_while_inner_return() -> Result<()> {
        let program = read_program("double_while_inner_return.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 3\n".to_string());
        Ok(())
    }

    #[test]
    fn empty_array() -> Result<()> {
        let program = read_program("empty_array.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str());

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn single_element_array() -> Result<()> {
        let program = read_program("single_element_array.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn array_mutation_through_function() -> Result<()> {
        let program = read_program("array_mutation_through_function.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 99\n$ 20\n$ 30\n".to_string());
        Ok(())
    }

    #[test]
    fn fill_array_through_function() -> Result<()> {
        let program = read_program("fill_array_through_function.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 7\n$ 7\n$ 7\n$ 7\n$ 7\n".to_string());
        Ok(())
    }

    #[test]
    fn sum_array_in_function() -> Result<()> {
        let program = read_program("sum_array_in_function.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 10\n".to_string());
        Ok(())
    }

    #[test]
    fn copy_array_between_arrays() -> Result<()> {
        let program = read_program("copy_array_between_arrays.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 8\n$ 6\n$ 7\n$ 5\n".to_string());
        Ok(())
    }

    #[test]
    fn clone_array() -> Result<()> {
        let program = read_program("clone_array.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 8\n$ 6\n$ 7\n$ 5\n".to_string());
        Ok(())
    }

    #[test]
    fn return_transformed_array_from_function() -> Result<()> {
        let program = read_program("return_transformed_array_from_function.tromb")?;

        let mock_allocator = MockAllocator {
            alloc_count: AtomicUsize::new(0),
        };
        let mut runner = setup_runner_with_alloc!("", &mock_allocator);

        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(
            result,
            "$ 138\n$ 8\n$ 16\n$ 30\n$ 2\n$ 14\n$ 38\n$ 6\n$ 24\n$ 12\n$ 4\n$ 100\n$ 200\n"
                .to_string()
        );

        // no leaks
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);

        Ok(())
    }

    #[test]
    fn chained_array_return_functions() -> Result<()> {
        let program = read_program("chained_array_return_functions.tromb")?;

        let mock_allocator = MockAllocator {
            alloc_count: AtomicUsize::new(0),
        };
        let mut runner = setup_runner_with_alloc!("", &mock_allocator);

        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 125\n$ 105\n$ 85\n$ 65\n$ 45\n$ 25\n".to_string());

        // no leaks
        assert_eq!(mock_allocator.alloc_count.load(Ordering::SeqCst), 0);

        Ok(())
    }

    #[test]
    fn reverse_array_in_place() -> Result<()> {
        let program = read_program("reverse_array_in_place.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn same_array_passed_twice() -> Result<()> {
        let program = read_program("same_array_passed_twice.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 16\n".to_string());
        Ok(())
    }

    #[test]
    fn recursive_array_access() -> Result<()> {
        let program = read_program("recursive_array_access.tromb")?;
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 11\n$ 22\n$ 33\n".to_string());
        Ok(())
    }

    #[test]
    fn factorial() -> Result<()> {
        let program = read_program("factorial.tromb")?;
        let mut runner = setup_runner!("5");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "> $ 120\n".to_string());

        let mut runner = setup_runner!("4");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "> $ 24\n".to_string());

        Ok(())
    }

    #[test]
    fn sieve() -> Result<()> {
        let program = read_program("sieve.tromb")?;
        let mut runner = setup_runner!("10");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "> $ 11\n$ 11\n$ 2\n$ 3\n$ 5\n$ 7\n".to_string());

        Ok(())
    }

    #[test]
    fn array_out_of_bounds() -> Result<()> {
        let program = read_program("array_out_of_bounds.tromb")?;

        let mut runner = setup_runner!("");
        let _result = run_test(&mut runner, program.as_str())?;

        // TODO: ??

        Ok(())
    }

    #[test]
    fn quicksort() -> Result<()> {
        let program = read_program("quicksort.tromb")?;
        let mut runner = setup_runner!("4\n3\n4\n2\n1");
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(result, "> > > > > $ 1\n$ 2\n$ 3\n$ 4\n".to_string());

        let mut runner = setup_runner!(
            "20\n17\n3\n14\n1\n19\n8\n6\n12\n4\n20\n2\n15\n9\n7\n13\n5\n18\n10\n11\n16"
        );
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(
            result,
            "> > > > > > > > > > > > > > > > > > > > > \
    $ 1\n$ 2\n$ 3\n$ 4\n$ 5\n$ 6\n$ 7\n$ 8\n$ 9\n$ 10\n$ 11\n$ 12\n$ 13\n$ 14\n$ 15\n$ 16\n$ 17\n$ 18\n$ 19\n$ 20\n"
                .to_string()
        );

        let mut runner = setup_runner!(
            "24\n5\n3\n8\n3\n1\n9\n5\n2\n8\n7\n3\n6\n4\n9\n1\n5\n2\n7\n6\n4\n8\n2\n9\n1"
        );
        let result = run_test(&mut runner, program.as_str())?;
        assert_eq!(
            result,
            "> > > > > > > > > > > > > > > > > > > > > > > > > \
    $ 1\n$ 1\n$ 1\n$ 2\n$ 2\n$ 2\n$ 3\n$ 3\n$ 3\n$ 4\n$ 4\n$ 5\n$ 5\n$ 5\n$ 6\n$ 6\n$ 7\n$ 7\n$ 8\n$ 8\n$ 8\n$ 9\n$ 9\n$ 9\n"
                .to_string()
        );

        Ok(())
    }

    #[test]
    fn quicksort_with_duplicates() -> Result<()> {
        let program = read_program("quicksort_with_duplicates.tromb")?;

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program.as_str())?;

        assert_eq!(result, "$ 1\n$ 1\n$ 2\n$ 2\n$ 5\n$ 7\n$ 9\n".to_string());
        Ok(())
    }
}
