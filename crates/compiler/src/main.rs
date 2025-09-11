use trombone_compiler_lib::*;

fn main() {
    let test = r"
fn main() {
    print(foo());
    return;
}

fn foo() -> int {
    return 42;
}
    ";

    let ops = compiler::compile_from_string(test.to_string());

    println!("Ops: {:?}", ops)
}
