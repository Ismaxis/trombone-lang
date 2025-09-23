use trombone_compiler_lib::*;

fn main() {
    let test = r"
fn main() {
    if 1 {
        print(1);
    }
    return;
}
    ";

    let ops = compiler::compile_from_string(test.to_string());

    ops.iter().for_each(|x| println!("{:?}", *x));
}
