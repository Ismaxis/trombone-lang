#[cfg(test)]
mod tests {
    use trombone_common::error::Result;

    use trombone_runner::runner::Runner;
    use trombone_runner::runner_test::test_utils::*;

    use trombone_compiler_lib::*;

    #[test]
    fn single_print() -> Result<()> {
        let program = r"
fn main() {
    print(40 + 2);
    return;
}
        ";

        let ops = compile_from_string(program.to_string());

        let input = std::io::Cursor::new("".as_bytes());
        let output = std::io::Cursor::new(Vec::new());

        let mut runner = Runner::new(
            SimpleTestOperationStream::new(),
            input,
            output,
            RunnerType::default_allocator(),
        );

        runner.stream.emplace_operations(ops);
        runner.evaluate()?;
        assert_eq!(
            String::from_utf8(runner.output.into_inner()).unwrap(),
            "$$ 42\n".to_string()
        );
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
