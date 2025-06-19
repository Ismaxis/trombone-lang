pub type OpCode = u8;

// Push instruction
// 63       56 55              32 31                           0
// +----------+------------------+-----------------------------+
// |   0x01   |      Unused      |       Immediate value       |
// +----------+------------------+-----------------------------+
pub const OP_PUSH: OpCode = 0x01;

// Pop instruction
//  63       56 55                                             0
//  +----------+-----------------------------------------------+
//  |   0x02   |                   Unused                      |
//  +----------+-----------------------------------------------+
pub const OP_POP: OpCode = 0x02;

// Copy and Store instructions
// 63       56 55              32 31                           0
// +----------+------------------+-----------------------------+
// |  0x03-04 |      Unused      |      Offset to variable     |
// +----------+------------------+-----------------------------+
pub const OP_LOCAL_COPY: OpCode = 0x03; // Reads Nth (N = offset) value from top of stack and pushes it on top of stack
pub const OP_LOCAL_STORE: OpCode = 0x04; // Pops top of stack and store it to Nth (N = offset) value from top of stack 

// Arithmetic instructions
//  63       56 55                                             0
//  +----------+-----------------------------------------------+
//  | 0xa0-ab  |                   Unused                      |
//  +----------+-----------------------------------------------+
pub const OP_NEG: OpCode = 0xa0;
pub const OP_NOT: OpCode = 0xa1;
pub const OP_ADD: OpCode = 0xa2;
pub const OP_SUB: OpCode = 0xa3;
pub const OP_MUL: OpCode = 0xa4;
pub const OP_DIV: OpCode = 0xa5;
pub const OP_MOD: OpCode = 0xa6;
pub const OP_AND: OpCode = 0xa7;
pub const OP_OR: OpCode = 0xa8;
pub const OP_XOR: OpCode = 0xa9;
pub const OP_LSH: OpCode = 0xaa;
pub const OP_RSH: OpCode = 0xab;

// Comparison instructions
//  63       56 55                                             0
//  +----------+-----------------------------------------------+
//  | 0xc0-c5  |                   Unused                      |
//  +----------+-----------------------------------------------+
pub const OP_EQ: OpCode = 0xc0; // Equal
pub const OP_NE: OpCode = 0xc1; // Not equal
pub const OP_LT: OpCode = 0xc2; // Less than
pub const OP_GT: OpCode = 0xc3; // Greater than
pub const OP_LE: OpCode = 0xc4; // Less than or equal
pub const OP_GE: OpCode = 0xc5; // Greater than or equal

// Jump instructions
// 63       56 55              32 31                           0
// +----------+------------------+-----------------------------+
// | 0xd0-d2  |      Unused      |         Jump Offset         |
// +----------+------------------+-----------------------------+
pub const OP_JMP: OpCode = 0xd0; // Unconditional jump
pub const OP_JMP_IF: OpCode = 0xd1; // Jump if true
pub const OP_JMP_IF_NOT: OpCode = 0xd2; // Jump if false

// Heap instructions
//   63       56 55                                              0
//   +----------+------------------------------------------------+
//   | 0xe0-e1  |                     Unused                     |
//   +----------+------------------------------------------------+
// Allocates region of memory, count of TromValues popped from stack
pub const OP_HEAP_ALLOC: OpCode = 0xe0;
// Decrements reference count, pops pointer from stack
pub const OP_HEAP_POP_PTR: OpCode = 0xe1;
//   63       56 55              32 31                           0
//   +----------+------------------+-----------------------------+
//   |  0xe2-e4 |      Unused      |      Offset to variable     |
//   +----------+------------------+-----------------------------+
// `ptr` - variable by offset
pub const OP_HEAP_COPY_PTR: OpCode = 0xe2; // Copies and pushes `ptr`, increments refcount
// `offset` - top of stack
// `ptr` - variable by offset
pub const OP_HEAP_LOAD_PTR: OpCode = 0xe3; // Pushes `ptr` + `offset`, increments refcount
// `offset` - top of stack
// `value` - second value on stack
// `ptr` - variable by offset
pub const OP_HEAP_STORE_PTR: OpCode = 0xe4; // Stores `value` to `ptr` + `offset`

// IO instructions
//  63       56 55                                             0
//  +----------+-----------------------------------------------+
//  | 0xf0-f1  |                   Unused                      |
//  +----------+-----------------------------------------------+
pub const OP_READ: OpCode = 0xf0; // Read next int from input stream and push to stack
pub const OP_PRINT: OpCode = 0xf1; // Pop value from stack and print to output stream
