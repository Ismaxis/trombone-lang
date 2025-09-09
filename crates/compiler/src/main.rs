use trombone_compiler_lib::*;

fn main() {
    let test = r"fn main() {
        print(2 + 2);
        return;
    }";

    let ops = compile_from_string(test.to_string());

    println!("Ops: {:?}", ops)
}
