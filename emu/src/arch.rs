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
}