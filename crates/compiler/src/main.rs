use trombone_compiler_lib::*;

fn main() {
    let test = r"
fn main() {
    foo(42);
    foo(42);
    foo(42);
    return;
}

fn foo(x: int) {
    print(x);
    return;
}
    ";

    let ops = compiler::compile_from_string(test.to_string());

    println!("Ops: {:?}", ops)
}
