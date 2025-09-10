use trombone_compiler_lib::*;

fn main() {
    let test = r"fn main() {
    let x: int = read();
    let y: int = read();
    let z: int = x + y;
    print(2 * x + y + z);
    return;
    }";

    let ops = compile_from_string(test.to_string());

    println!("Ops: {:?}", ops)
}
