pub type OpCode = u8;

// Push instructions
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
//  | 0x10-1b  |                   Unused                      |
//  +----------+-----------------------------------------------+
pub const OP_NEG: OpCode = 0x10;
pub const OP_NOT: OpCode = 0x11;
pub const OP_ADD: OpCode = 0x12;
pub const OP_SUB: OpCode = 0x13;
pub const OP_MUL: OpCode = 0x14;
pub const OP_DIV: OpCode = 0x15;
pub const OP_MOD: OpCode = 0x16;
pub const OP_AND: OpCode = 0x17;
pub const OP_OR: OpCode = 0x18;
pub const OP_XOR: OpCode = 0x19;
pub const OP_LSH: OpCode = 0x1a;
pub const OP_RSH: OpCode = 0x1b;

// Comparison instructions
//  63       56 55                                             0
//  +----------+-----------------------------------------------+
//  | 0x80-85  |                   Unused                      |
//  +----------+-----------------------------------------------+
pub const OP_EQ: OpCode = 0x80; // Equal
pub const OP_NE: OpCode = 0x81; // Not equal
pub const OP_LT: OpCode = 0x82; // Less than
pub const OP_GT: OpCode = 0x83; // Greater than
pub const OP_LE: OpCode = 0x84; // Less than or equal
pub const OP_GE: OpCode = 0x85; // Greater than or equal

// Jump instructions
// 63       56 55              32 31                           0
// +----------+------------------+-----------------------------+
// | 0x90-92  |      Unused      |         Jump Offset         |
// +----------+------------------+-----------------------------+
pub const OP_JMP: OpCode = 0x90; // Unconditional jump
pub const OP_JMP_IF: OpCode = 0x91; // Jump if true
pub const OP_JMP_IF_NOT: OpCode = 0x92; // Jump if false
