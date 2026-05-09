use self::Opcode::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Opcode {
    /// Stop all execution instantly.
    Halt,
    // Load a 64-bit floating-point value into a register.
    Load,
    /// Move a value from a register to a memory location or vice versa. The first
    /// argument is the source, the second is the destination. Swap the arguments
    /// to change the direction. The source will be cleared after the operation.
    ///
    /// This opcode is followed by two 64-bit unsigned integers.
    Move,
    /// Copy a value from a register to a memory location or vice versa.
    /// The source will be left untouched.
    ///
    /// This opcode is followed by two 64-bit unsigned integers.
    Copy,
    /// Perform a calculation (`ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`) on two registers
    /// and write the result to a third.
    ///
    /// This opcode is followed by three 16-bit unsigned integers.
    Calculate,
    /// Perform a comparison (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`) on two registers
    /// and write the result to a third.
    ///
    /// This opcode is followed by three 16-bit unsigned integers.
    Compare,
    /// Perform a bitwise operation (`AND`, `OR`, `XOR`, `NOT`, `LSH`, `RSH`) on two or
    /// three addresses.
    Bitwise,
    /// Set the program counter to a value from a location, using one of the
    /// methods (`absolute`, `forward`, `backward`)
    Jump,
    /// Set the program counter to a value from a location if the byte at a given
    /// address in memory has the value of 1
    JumpC,
    /// Emit an event that needs immediate attention (`READ`, `PRINT`)
    Interrupt,
    Call,
    Return,
    Push,
    Append,
    /// Store a register value into the current stack frame's local variables.
    StoreLocal,
    /// Load a value from the current stack frame's local variables into a register.
    LoadLocal,
    /// Dispatch a multimethod call based on runtime argument value.
    /// Reads from `arg` register, matches against patterns, jumps to matching method.
    Dispatch,
    /// An invalid opcode.
    Illegal,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Opcode {
        match byte {
            0 => Halt,
            1 => Load,
            2 => Move,
            3 => Copy,
            4 => Calculate,
            5 => Compare,
            6 => Bitwise,
            7 => Jump,
            8 => JumpC,
            9 => Interrupt,
            10 => Call,
            11 => Return,
            12 => Push,
            13 => Append,
            14 => StoreLocal,
            15 => LoadLocal,
            16 => Dispatch,

            _ => Illegal,
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Halt => 0,
            Load => 1,
            Move => 2,
            Copy => 3,
            Calculate => 4,
            Compare => 5,
            Bitwise => 6,
            Jump => 7,
            JumpC => 8,
            Interrupt => 9,
            Call => 10,
            Return => 11,
            Push => 12,
            Append => 13,
            StoreLocal => 14,
            LoadLocal => 15,
            Dispatch => 16,

            Illegal => 255,
        }
    }
}
