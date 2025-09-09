use trombone_compiler_lib::*;

fn main() {
    let test = r"fn main() {
        let x: int = read();
        print(x);
        return;
    }";

    let ops = compile_from_string(test.to_string());

    println!("Ops: {:?}", ops)
}
