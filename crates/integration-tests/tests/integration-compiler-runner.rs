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
        let ops = compiler::compile_from_string(program.to_string())?;
        runner.stream.emplace_operations(ops);
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
    fn function_call_with_return() -> Result<()> {
        let program = r"
fn main() {
    print(foo());
    print(foo());
    print(foo());
    return;
}

fn foo() -> int {
    return 42;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n$ 42\n$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_discard() -> Result<()> {
        let program = r"
fn main() {
    foo();
    foo();
    foo();
    return;
}

fn foo() -> int {
    read();
    return 42;
}
        ";
        let mut runner = setup_runner!("1\n1\n1\n");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "> > > ".to_string());
        Ok(())
    }

    #[test]
    fn function_call_inc() -> Result<()> {
        let program = r"
fn main() {
    print(foo(42));
    print(foo(43) + foo(44));
    return;
}

fn foo(x: int) -> int {
    return x + 1;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 43\n$ 89\n".to_string());
        Ok(())
    }

    #[test]
    fn function_call_sum() -> Result<()> {
        let program = r"
fn main() {
    print(foo(1, 2));
    print(foo(3, 4));
    print(foo(4, 6));
    return;
}

fn foo(x: int, y: int) -> int {
    return x + y;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 3\n$ 7\n$ 10\n".to_string());
        Ok(())
    }

    // If

    #[test]
    fn basic_if() -> Result<()> {
        let program = r"
fn main() {
    if 1 {
        print(1);
    }
    if 0 {
        print(2);
    }
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn nested_if() -> Result<()> {
        let program = r"
fn main() {
    if 1 {
        if 1 {
            print(1);
        }
        if 0 {
            print(2);
        }
    }
    if 0 {
        if 1 {
            print(3);
        }
        if 0 {
            print(4);
        }
    }
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn if_return() -> Result<()> {
        let program = r"
fn main() {
    print(is_zero(0));
    print(is_zero(1));
    print(is_zero(2));

    return;
}

fn is_zero(x: int) -> int {
    if x == 0 {
        return 1;
    } else {
        return 0;
    }
    return -1;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n$ 0\n$ 0\n".to_string());
        Ok(())
    }

    #[test]
    fn if_return_hard() -> Result<()> {
        let program = r"
fn main() {
    print(foo(1, 1));
    print(foo(1, 0));
    print(foo(0, 1));
    print(foo(0, 0));
    return;
}

fn foo(x: int, y: int) -> int {
    if x {
        if y {
            return 3;
        } else {
            return 2;
        }
    } else {
        if y {
            return 1;
        } else {
            return 0;
        }
    }
    return -1;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 3\n$ 2\n$ 1\n$ 0\n".to_string());
        Ok(())
    }

    #[test]
    fn if_return_deep() -> Result<()> {
        let program = r"
fn main() {
    print(bar(1, 2, 1));
    print(bar(1, 10, 1));
    print(bar(1, 10, 0));
    return;
}

fn bar(x: int, y: int, z: int) -> int {
    let x1: int = x;
    if x == x1 {
        let y1: int = y;
        if y == y1 {
            let z1: int = z;
            if z == 1 {
                return y1;
            }
        }
    }
    return -1;
}
        ";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 2\n$ 10\n$ -1\n".to_string());
        Ok(())
    }

    #[test]
    fn if_clear_locals() -> Result<()> {
        let program = r"
fn main() {
    print(bar(1, 2, 1));
    print(bar(1, 10, 1));
    print(bar(1, -1, 0));
    return;
}

fn bar(x: int, y: int, z: int) -> int {
    if 1 {
        let x1: int = x;
    }
    return y;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 2\n$ 10\n$ -1\n".to_string());
        Ok(())
    }

    #[test]
    fn recursion() -> Result<()> {
        let program = r"
fn main() {
    rec(5);
    return;
}

fn rec(x: int) {
    if x == 0 {
        return;
    } 

    print(x);
    rec(x - 1);
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn basic_if_else() -> Result<()> {
        let program = r"
fn main() {
    if 1 {
        print(1);
    } else {
        print(2);
    }
    
    if 0 {
        print(1);
    } else {
        print(2);
    }

    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n$ 2\n".to_string());
        Ok(())
    }

    #[test]
    fn if_chain() -> Result<()> {
        let program = r"
    fn main() {
        foo(1);
        foo(2);
        foo(3);
        foo(10);
        return;
    }

    fn foo(x: int) {
        if x == 1 {
            print(1);
        } else if x == 2 {
            print(2);
        } else if x == 3 {
            print(3);
        } else {
            print(4);
        }
        return;
    }
            ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n$ 2\n$ 3\n$ 4\n".to_string());
        Ok(())
    }

    #[test]
    fn if_chain_no_else() -> Result<()> {
        let program = r"
    fn main() {
        foo(1);
        foo(2);
        foo(10);
        return;
    }

    fn foo(x: int) {
        if x == 1 {
            print(1);
        } else if x == 2 {
            print(2);
        }

        print(3);
        return;
    }
            ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n$ 3\n$ 2\n$ 3\n$ 3\n".to_string());
        Ok(())
    }

    #[test]
    fn simple_while() -> Result<()> {
        let program = r"
fn main() {
    let x: int = 10;
    while x > 0 {
        print(x);
        x = x - 1;
    }
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(
            result,
            "$ 10\n$ 9\n$ 8\n$ 7\n$ 6\n$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string()
        );
        Ok(())
    }

    #[test]
    fn while_no_execution() -> Result<()> {
        let program = r"
fn main() {
    let x: int = 2;
    while x != 2 {
        print(1337);
    }
    print(42);
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn inner_while() -> Result<()> {
        let program = r"
fn main() {
    let x: int = 5;
    while x > 0 {
        let y: int = 3;
        while y > 0 {
            print(x * y);
            y = y - 1;
        }
        x = x - 1;
    }
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(
            result,
            "$ 15\n$ 10\n$ 5\n$ 12\n$ 8\n$ 4\n$ 9\n$ 6\n$ 3\n$ 6\n$ 4\n$ 2\n$ 3\n$ 2\n$ 1\n"
                .to_string()
        );
        Ok(())
    }

    #[test]
    fn return_in_while() -> Result<()> {
        let program = r"
fn main() {
    let x: int = read();
    while x == 0 {
        print(0);
        return;
    }
    print(1);
    return;
}
        ";
        let mut runner = setup_runner!("0");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "> $ 0\n".to_string());

        let mut runner = setup_runner!("123");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "> $ 1\n".to_string());

        Ok(())
    }

    #[test]
    fn return_nested_if_and_while() -> Result<()> {
        let program = r"
fn main() {
    print(while_in_if(1));
    print(while_in_if(2));
    print(if_in_while(1));
    print(if_in_while(2));
    return;
}

fn while_in_if(x: int) -> int {
    if x {
        while x == 1 {
            return 1;
        }
        return 0;
    }
    return -1;
}

fn if_in_while(x: int) -> int {
    while x {
        if x == 1 {
            return 1;
        }
        return 0;
    }
    return -1;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "$ 1\n$ 0\n$ 1\n$ 0\n".to_string());

        Ok(())
    }

    #[test]
    fn while_condition_is_function_call() -> Result<()> {
        let program = r"
fn main() {
    let x: int = 5;
    while foo(x) > 0 {
        print(x);
        x = x - 1;
    }
    return;
}

fn foo(x: int) -> int {
    return x > 0;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn double_while_inner_return() -> Result<()> {
        let program = r"
fn main() {
    let x: int = 5;
    let y: int = 3;
    while x > 0 {
        while y > 0 {
            print(y);
            return;
        }
        x = x - 1;
    }
    return;
}
        ";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 3\n".to_string());
        Ok(())
    }

    #[test]
    fn empty_array() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(0);
    print(123);
    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn single_element_array() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(1);

    arr[0] = 42;
    print(arr[0]);

    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 42\n".to_string());
        Ok(())
    }

    #[test]
    fn array_mutation_through_function() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(3);

    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;

    set_first(arr, 99);

    print(arr[0]);
    print(arr[1]);
    print(arr[2]);

    return;
}

fn set_first(arr: [int], value: int) {
    arr[0] = value;
    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 99\n$ 20\n$ 30\n".to_string());
        Ok(())
    }

    #[test]
    fn fill_array_through_function() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(5);

    fill(arr, 7, 5);

    let i: int = 0;
    while i < 5 {
        print(arr[i]);
        i = i + 1;
    }

    return;
}

fn fill(arr: [int], value: int, n: int) {
    let i: int = 0;

    while i < n {
        arr[i] = value;
        i = i + 1;
    }

    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 7\n$ 7\n$ 7\n$ 7\n$ 7\n".to_string());
        Ok(())
    }

    #[test]
    fn sum_array_in_function() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(4);

    arr[0] = 3;
    arr[1] = 4;
    arr[2] = 2;
    arr[3] = 1;

    let result: int = sum(arr, 4);
    print(result);

    return;
}

fn sum(arr: [int], n: int) -> int {
    let total: int = 0;
    let i: int = 0;

    while i < n {
        total = total + arr[i];
        i = i + 1;
    }

    return total;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 10\n".to_string());
        Ok(())
    }

    #[test]
    fn copy_array_between_arrays() -> Result<()> {
        let program = r"
fn main() {
    let source: [int] = array(4);
    let destination: [int] = array(4);

    source[0] = 8;
    source[1] = 6;
    source[2] = 7;
    source[3] = 5;

    copy_array(source, destination, 4);

    let i: int = 0;
    while i < 4 {
        print(destination[i]);
        i = i + 1;
    }

    return;
}

fn copy_array(source: [int], destination: [int], n: int) {
    let i: int = 0;

    while i < n {
        destination[i] = source[i];
        i = i + 1;
    }

    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 8\n$ 6\n$ 7\n$ 5\n".to_string());
        Ok(())
    }

    #[test]
    fn reverse_array_in_place() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(5);

    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;
    arr[3] = 4;
    arr[4] = 5;

    reverse(arr, 5);

    let i: int = 0;
    while i < 5 {
        print(arr[i]);
        i = i + 1;
    }

    return;
}

fn reverse(arr: [int], n: int) {
    let left: int = 0;
    let right: int = n - 1;

    while left < right {
        let temp: int = arr[left];
        arr[left] = arr[right];
        arr[right] = temp;

        left = left + 1;
        right = right - 1;
    }

    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 5\n$ 4\n$ 3\n$ 2\n$ 1\n".to_string());
        Ok(())
    }

    #[test]
    fn same_array_passed_twice() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(1);
    arr[0] = 5;

    modify(arr, arr);

    print(arr[0]);

    return;
}

fn modify(a: [int], b: [int]) {
    a[0] = a[0] + 1;
    b[0] = b[0] + 10;
    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 16\n".to_string());
        Ok(())
    }

    #[test]
    fn recursive_array_access() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(3);

    arr[0] = 11;
    arr[1] = 22;
    arr[2] = 33;

    print_forward(arr, 0, 3);

    return;
}

fn print_forward(arr: [int], index: int, n: int) {
    if index < n {
        print(arr[index]);
        print_forward(arr, index + 1, n);
    }

    return;
}
";
        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 11\n$ 22\n$ 33\n".to_string());
        Ok(())
    }

    #[test]
    fn factorial() -> Result<()> {
        let program = r"
fn main() {
    let n: int = read();
    print(factorial(n));
    return;
}

fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
    ";
        let mut runner = setup_runner!("5");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "> $ 120\n".to_string());

        let mut runner = setup_runner!("4");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "> $ 24\n".to_string());

        Ok(())
    }

    #[test]
    fn sieve() -> Result<()> {
        let program = r"
fn main() {
    let n: int = read();
    sieve(n);
    return;
}

fn sieve(n: int) {
    let t: int = n + 1;
    print(t);
    let is_prime: [int] = array(t);
    print(t);

    let cnt: int = 0;
    while cnt <= n {
        is_prime[cnt] = 1;
        cnt = cnt + 1;
    }

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
    return;
}
";
        let mut runner = setup_runner!("10");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "> $ 11\n$ 11\n$ 2\n$ 3\n$ 5\n$ 7\n".to_string());

        Ok(())
    }

    #[test]
    fn array_out_of_bounds() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(3);

    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;

    arr[3] = 40;

    return;
}
";

        let mut runner = setup_runner!("");
        let _result = run_test(&mut runner, program)?;

        // TODO: ??

        Ok(())
    }

    #[test]
    fn quicksort() -> Result<()> {
        let program = r"
fn main() {
    let n: int = read();
    let arr: [int] = array(n);
    let i: int = 0;
    while i < n {
        arr[i] = read();
        i = i + 1;
    }

    quicksort(arr, 0, n - 1);

    i = 0;
    while i < n {
        print(arr[i]);
        i = i + 1;
    }

    return;
}

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

        j = j + 1;
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

    return;
}
";
        let mut runner = setup_runner!("4\n3\n4\n2\n1");
        let result = run_test(&mut runner, program)?;
        assert_eq!(result, "> > > > > $ 1\n$ 2\n$ 3\n$ 4\n".to_string());

        let mut runner = setup_runner!(
            "20\n\
         17\n\
         3\n\
         14\n\
         1\n\
         19\n\
         8\n\
         6\n\
         12\n\
         4\n\
         20\n\
         2\n\
         15\n\
         9\n\
         7\n\
         13\n\
         5\n\
         18\n\
         10\n\
         11\n\
         16"
        );
        let result = run_test(&mut runner, program)?;
        assert_eq!(
            result,
            "> > > > > > > > > > > > > > > > > > > > > \
         $ 1\n\
         $ 2\n\
         $ 3\n\
         $ 4\n\
         $ 5\n\
         $ 6\n\
         $ 7\n\
         $ 8\n\
         $ 9\n\
         $ 10\n\
         $ 11\n\
         $ 12\n\
         $ 13\n\
         $ 14\n\
         $ 15\n\
         $ 16\n\
         $ 17\n\
         $ 18\n\
         $ 19\n\
         $ 20\n"
                .to_string()
        );

        let mut runner = setup_runner!(
            "24\n\
         5\n\
         3\n\
         8\n\
         3\n\
         1\n\
         9\n\
         5\n\
         2\n\
         8\n\
         7\n\
         3\n\
         6\n\
         4\n\
         9\n\
         1\n\
         5\n\
         2\n\
         7\n\
         6\n\
         4\n\
         8\n\
         2\n\
         9\n\
         1"
        );
        let result = run_test(&mut runner, program)?;
        assert_eq!(
            result,
            "> > > > > > > > > > > > > > > > > > > > > > > > > \
         $ 1\n\
         $ 1\n\
         $ 1\n\
         $ 2\n\
         $ 2\n\
         $ 2\n\
         $ 3\n\
         $ 3\n\
         $ 3\n\
         $ 4\n\
         $ 4\n\
         $ 5\n\
         $ 5\n\
         $ 5\n\
         $ 6\n\
         $ 6\n\
         $ 7\n\
         $ 7\n\
         $ 8\n\
         $ 8\n\
         $ 8\n\
         $ 9\n\
         $ 9\n\
         $ 9\n"
                .to_string()
        );

        Ok(())
    }

    #[test]
    fn quicksort_with_duplicates() -> Result<()> {
        let program = r"
fn main() {
    let arr: [int] = array(7);

    arr[0] = 2;
    arr[1] = 7;
    arr[2] = 2;
    arr[3] = 9;
    arr[4] = 1;
    arr[5] = 5;
    arr[6] = 1;

    quicksort(arr, 0, 6);

    let i: int = 0;
    while i < 7 {
        print(arr[i]);
        i = i + 1;
    }

    return;
}

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

        j = j + 1;
    }

    let temp: int = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;

    return i + 1;
}

fn quicksort(arr: [int], low: int, high: int) {
    if low < high {
        let pi: int = partition(arr, low, high);

        if low < pi {
            quicksort(arr, low, pi - 1);
        }

        if pi < high {
            quicksort(arr, pi + 1, high);
        }
    }

    return;
}
";

        let mut runner = setup_runner!("");
        let result = run_test(&mut runner, program)?;

        assert_eq!(result, "$ 1\n$ 1\n$ 2\n$ 2\n$ 5\n$ 7\n$ 9\n".to_string());
        Ok(())
    }
}
