pub type Register = u8;
// pub type Literal = u32; // stores address in heap

pub enum OpCode {
    Add {
        dest: Register,
        src1: Register,
        src2: Register,
    },
    // LoadLiteral {
    //     dest: Register,
    //     src: Literal
    // },
    // StoreLiteral {
    //     dest: Literal,
    //     src: Register
    // }
}

pub type ArrayOpCode = std::vec::Vec<OpCode>;
