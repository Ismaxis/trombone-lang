#[cfg(test)]
mod tests {
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
        runner
            .stream
            .emplace_operations(compiler::compile_from_string(program.to_string()));
        runner.evaluate()?;

        Ok(String::from_utf8(runner.output.clone().into_inner()).unwrap())
    }

    #[test]
    fn single_print() -> Result<()> {
        let program = r"
fn main() {
    print(40 + 2);
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn read_and_print() -> Result<()> {
        let program = r"
fn main() {
    let x: int = read();
    print(x);
    return;
}
        ";
        let mut runner = setup_runner!("42");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "> $ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn variables_access() -> Result<()> {
        let program = r"
fn main() {
    let x: int = read();
    let y: int = read();
    let z: int = x + y;
    let a: int = z * z * x * y;
    print(a * (2 * x + y + z));
    return;
}
        ";
        let mut runner = setup_runner!("10\n100");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "> > $ 2783000000\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call() -> Result<()> {
        let program = r"
fn main() {
    foo();
    foo();
    foo();
    return;
}

fn foo() {
    print(42);
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n$ 42\n$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_with_arguments() -> Result<()> {
        let program = r"
fn main() {
    foo(42);
    foo(43);
    foo(44);
    return;
}

fn foo(x: int) {
    print(x);
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n$ 43\n$ 44\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_with_multiple_arguments() -> Result<()> {
        let program = r"
fn main() {
    foo(42, 43, 44);
    foo(52, 53, 54);
    return;
}

fn foo(x: int, y: int, z: int) {
    print(x);
    print(y);
    print(z);
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n$ 43\n$ 44\n$ 52\n$ 53\n$ 54\n".to_string());
        Ok(())
    }

    #[test]
    fn factorial() {
        let test = r"
        fn factorial(n: int) -> int {
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }

        fn main() {
            let n: int = read();
            print(factorial(n));
            return;
        }
    ";

        let res = trombone::ProgramParser::new().parse(test);
        assert!(res.is_ok(), "{}", res.unwrap_err());
        // TODO: check execution
    }

    #[test]
    fn sieve() {
        let test = r"
fn sieve(n: int) {
    let is_prime: [int] = array(n + 1, 1);
    is_prime[0] = 0;
    is_prime[1] = 0;

    let i: int = 2;
    let j: int = 0;
    while i * i <= n {
        if is_prime[i] {
            let j: int = i * i;
            while j <= n {
                is_prime[j] = 0;
                j = j + i;
            }
        }
        i = i + 1;
    }

    i = 0;
    while i <= n {
        if is_prime[i] {
            print(i);
        }
        i = i + 1;
    }
}

fn main() {
    let n: int = read();
    sieve(n);
}
";

        let res = trombone::ProgramParser::new().parse(test);
        assert!(res.is_ok(), "{}", res.unwrap_err());
        // TODO: check execution
    }

    #[test]
    fn sort() {
        let test = r"
fn partition(arr: [int], low: int, high: int) -> int {
    let pivot: int = arr[high];
    let i: int = low - 1;
    let j: int = low;

    while j < high {
        if arr[j] <= pivot {
            i = i + 1;
            let temp: int = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }

    let temp: int = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;

    return i + 1;
}

fn quicksort(arr: [int], low: int, high: int) {
    if low < high {
        let pi: int = partition(arr, low, high);

        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

fn main() {
    let n: int = read();
    let arr: [int] = array(n);
    let i: int = 0;
    while i < n {
        arr[i] = read();
        i = i + 1;
    }

    i = 0;
    while i < n {
        print(arr[i]);
        i = i + 1;
    }
}
";

        let res = trombone::ProgramParser::new().parse(test);
        assert!(res.is_ok(), "{}", res.unwrap_err());
        // TODO: check execution
    }
}
