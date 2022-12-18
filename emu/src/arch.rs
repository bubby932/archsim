/// # Architecture Instruction Set
/// Instruction opcodes are ALWAYS 1 byte for performance and complexity reasons.
/// Memory addresses in parameters may extend the overall size of a single operation to up to 2 bytes.
/// 
/// # Instructions
/// * `NOP` - `0x00` - `0000 0000` - No-op instruction. Used for padding, testing, or delays.
/// 
/// * `MOVAB` - `0x01` - `0000 0001` - Moves REG_B8 into REG_A8
/// * `MOVBA` - `0x02` - `0000 0010` - Moves REG_A8 into REG_B8
/// * `MOVAC` - `0x03` - `0000 0011` - Moves REG_C8 into REG_A8
/// * `MOVBC` - `0x04` - `0000 0100` - Moves REG_C8 into REG_B8
/// * `MOVCA` - `0x05` - `0000 0101` - Moves REG_A8 into REG_C8
/// * `MOVCB` - `0x06` - `0000 0110` - Moves REG_B8 into REG_C8
/// 
/// * `INCA` - `0x07` - `0000 0111` - Increments REG_A8 using the ALU.
/// * `INCB` - `0x08` - `0000 1000` - Increments REG_B8 using the ALU.
/// * `INCC` - `0x09` - `0000 1001` - Increments REG_C8 using the ALU.

#[repr(u8)]
pub enum Instruction {
    /// `NOP` - `0x00` - `0000 0000` - No-op instruction. Used for padding, testing, or delays.
    NOP = 0x00,

    /// `MOVAB` - `0x01` - `0000 0001` - Moves REG_B8 into REG_A8.
    MOVAB = 0x01,
    /// `MOVBA` - `0x02` - `0000 0010` - Moves REG_A8 into REG_B8.
    MOVBA = 0x02,
    /// `MOVAC` - `0x03` - `0000 0011` - Moves REG_C8 into REG_A8.
    MOVAC = 0x03,
    /// `MOVBC` - `0x04` - `0000 0100` - Moves REG_C8 into REG_B8.
    MOVBC = 0x04,
    /// `MOVCA` - `0x05` - `0000 0101` - Moves REG_A8 into REG_C8.
    MOVCA = 0x05,
    /// `MOVCB` - `0x06` - `0000 0110` - Moves REG_B8 into REG_C8.
    MOVCB = 0x06,

    /// `INCA` - `0x07` - `0000 0111` - Increments REG_A8 using the ALU.
    INCA = 0x07,
    /// `INCB` - `0x08` - `0000 1000` - Increments REG_B8 using the ALU.
    INCB = 0x08,
    /// `INCC` - `0x09` - `0000 1001` - Increments REG_C8 using the ALU.
    INCC = 0x09,

    /// `CMPAB` - `0x0A` - `0000 1010` - Compares A >|=|< B using the ALU.
    CMPAB = 0x0A,
    /// `CMPAC` - `0x0B` - `0000 1011` - Compares A >|=|< C using the ALU.
    CMPAC = 0x0B,
    /// `CMPBC` - `0x0C` - `0000 1100` - Compares B >|=|< C using the ALU
    CMPBC = 0x0C,

    /// `ADDAB` - `0x0D` - `0000 1101` - Adds REG_B8 to REG_A8 and places the result in REG_A8.
    ADDAB = 0x0D,
    /// `ADDAC` - `0x0E` - `0000 1110` - Adds REG_C8 to REG_A8 and places the result in REG_A8.
    ADDAC = 0x0E,
    /// `ADDBC` - `0x0F` - `0000 1111` - Adds REG_C8 to REG_B8 and places the result in REG_B8.
    ADDBC = 0x0F,
    /// `ADDBA` - `0x10` - `0001 0000` - Adds REG_A8 to REG_B8 and places the result in REG_B8.
    ADDBA = 0x10,
    /// `ADDCA` - `0x11` - `0001 0001` - Adds REG_A8 to REG_C8 and places the result in REG_C8.
    ADDCA = 0x11,
    /// `ADDCB` - `0x12` - `0001 0010` - Adds REG_B8 to REG_C8 and places the result in REG_C8.
    ADDCB = 0x12,

    /// `SUBAB` - `0x13` - `0001 0011` - Subtracts REG_B8 from REG_A8 and places the result in REG_A8.
    SUBAB = 0x13,
    /// `SUBAC` - `0x14` - `0001 0100` - Subtracts REG_C8 from REG_A8 and places the result in REG_A8.
    SUBAC = 0x14,
    /// `SUBBA` - `0x15` - `0001 0101` - Subtracts REG_A8 from REG_B8 and places the result in REG_B8.
    SUBBA = 0x15,
    /// `SUBBC` - `0x16` - `0001 0110` - Subtracts REG_C8 from REG_B8 and places the result in REG_B8.
    SUBBC = 0x16,
    /// `SUBCA` - `0x17` - `0001 0111` - Subtracts REG_A8 from REG_C8 and places the result in REG_C8.
    SUBCA = 0x17,
    /// `SUBCB` - `0x18` - `0001 1000` - Subtracts REG_B8 from REG_C8 and places the result in REG_C8.
    SUBCB = 0x18,

    /// `DUMP` - `0xFF` - `1111 1111` - EMULATOR ONLY - Prints the value of all registers.
    DUMP = 0xFF,
}

impl From<u8> for Instruction {
    fn from(u: u8) -> Self {
        match u {
            0x00 => Self::NOP,

            0x01 => Self::MOVAB,
            0x02 => Self::MOVBA,
            0x03 => Self::MOVAC,
            0x04 => Self::MOVBC,
            0x05 => Self::MOVCA,
            0x06 => Self::MOVCB,

            0x07 => Self::INCA,
            0x08 => Self::INCB,
            0x09 => Self::INCC,

            0x0A => Self::CMPAB,
            0x0B => Self::CMPAC,
            0x0C => Self::CMPBC,

            0x0D => Self::ADDAB,
            0x0E => Self::ADDAC,
            0x0F => Self::ADDBC,
            0x10 => Self::ADDBA,
            0x11 => Self::ADDCA,
            0x12 => Self::ADDCB,

            0x13 => Self::SUBAB,
            0x14 => Self::SUBAC,
            0x15 => Self::SUBBA,
            0x16 => Self::SUBBC,
            0x17 => Self::SUBCA,
            0x18 => Self::SUBCB,

            0xFF => Self::DUMP,
            _ => Self::DUMP
        }
    }
}

/// # Used for accessing the bit flags of REG_FLAGS in a safe manner.
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Flags {
    /// Set when an ALU operation has a carry at the end or otherwise overflows.
    AluCarry    = 0b_0000_0001,
    /// Set when the result of the last CMP\*\* operation is Greater or Equal.
    CmpGreater  = 0b_0000_0010,
    /// Set when the result of the last CMP\*\* operation is Greater/Less or Equal.
    CmpEqual    = 0b_0000_0100,
    /// Set when the result of the last CMP\*\* operation is Less or Equal.
    CmpLess     = 0b_0000_1000,
}