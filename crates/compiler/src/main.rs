pub mod ast;
pub mod error;
pub mod instruction_writer;
pub mod visit;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub trombone);

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
}

fn main() {
    let test = r"fn main() {
        print(2 + 2);
    }";

    println!("Input: {}", test);

    let res = trombone::ProgramParser::new().parse(test);
    if res.is_err() {
        println!("Error: {}", res.unwrap_err())
    } else {
        println!("Output:");
        println!("{:#?}", res.unwrap());
    }
}
