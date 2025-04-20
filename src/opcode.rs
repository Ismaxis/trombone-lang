pub type OpCode = u8;

// Comparison opcodes
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

// Jump opcodes
// 63       56 55              32 31                           0
// +----------+------------------+-----------------------------+
// | 0x90-92  |      Unused      |         Jump Offset         |
// +----------+------------------+-----------------------------+
pub const OP_JMP: OpCode = 0x90; // Unconditional jump
pub const OP_JMP_IF: OpCode = 0x91; // Jump if true
pub const OP_JMP_IF_NOT: OpCode = 0x92; // Jump if false
