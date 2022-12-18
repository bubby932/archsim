use crate::arch::{Flags, Instruction};

/// # Machine Emulator
/// This struct contains all pertinent information about the system, for use by the emulator.
pub struct Machine {
    /// Represented as REG_A8 in instructions, also referred to as the Accumulator.
    pub reg_a8 : u8,
    /// Represented as REG_B8 in instructions, also referred to as the Data Register.
    pub reg_b8 : u8,
    /// Represented as REG_C8 in instructions, also referred to as the Counter.
    pub reg_c8 : u8,
    /// Represented as REG_I16 in instructions, also referred to as the Instruction Pointer.
    pub reg_ic : u16,
    /// Represented as REG_FLAGS in instructions, also referred to as the Flags Register.
    pub reg_flags : u8,
    /// The entire working memory of the system. 65,535 bytes or 65.5KB if you prefer.
    pub memory : Box<[u8]>
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machine Status Dump:")?;
        writeln!(f, "  Registers:")?;
        writeln!(f, "    REG_A8: {}", self.reg_a8)?;
        writeln!(f, "    REG_B8: {}", self.reg_b8)?;
        writeln!(f, "    REG_C8: {}", self.reg_c8)?;
        writeln!(f, "  Flags:")?;
        writeln!(f, "    Overflow: {}", (self.reg_flags & (Flags::AluCarry as u8)) == {Flags::AluCarry as u8})?;
        writeln!(f, "    CmpGreater: {}", (self.reg_flags & (Flags::CmpGreater as u8)) == {Flags::CmpGreater as u8})?;
        writeln!(f, "    CmpEqual: {}", (self.reg_flags & (Flags::CmpEqual as u8)) == {Flags::CmpEqual as u8})?;
        writeln!(f, "    CmpLess: {}", (self.reg_flags & (Flags::CmpLess as u8)) == {Flags::CmpLess as u8})?;
        writeln!(f, "  Instruction Pointer: {}", self.reg_ic)?;

        Ok(())
    }
}

impl Machine {
    pub fn new(mem : Box<[u8]>) -> Self {
        Self {
            reg_a8: 0b_0000_0000,
            reg_b8: 0b_0000_0000,
            reg_c8: 0b_0000_0000,
            reg_ic: 0b_0000_0000,
            reg_flags: 0b_0000_0000,
            memory: mem
        }
    }

    pub fn set_flag(&mut self, flag : Flags, value : bool) {
        if value {
            self.reg_flags = (self.reg_flags & !(flag as u8)) | (flag as u8);
        } else {
            self.reg_flags = self.reg_flags & !(flag as u8);
        }
    }

    pub fn exec(&mut self) -> ! {
        loop {
            let opcode = Instruction::from(self.memory[self.reg_ic as usize]);
            self.reg_ic += 1;

            match opcode {
                Instruction::NOP => continue,

                Instruction::MOVAB => self.reg_a8 = self.reg_b8,
                Instruction::MOVBA => self.reg_b8 = self.reg_a8,
                Instruction::MOVAC => self.reg_a8 = self.reg_c8,
                Instruction::MOVBC => self.reg_b8 = self.reg_c8,
                Instruction::MOVCA => self.reg_c8 = self.reg_a8,
                Instruction::MOVCB => self.reg_c8 = self.reg_b8,

                Instruction::INCA => self.reg_a8 += 1,
                Instruction::INCB => self.reg_b8 += 1,
                Instruction::INCC => self.reg_c8 += 1,

                Instruction::CMPAB => {
                    self.set_flag(Flags::CmpGreater, self.reg_a8 > self.reg_b8);
                    self.set_flag(Flags::CmpEqual, self.reg_a8 == self.reg_b8);
                    self.set_flag(Flags::CmpLess, self.reg_a8 < self.reg_b8);
                },
                Instruction::CMPAC => {
                    self.set_flag(Flags::CmpGreater, self.reg_a8 > self.reg_c8);
                    self.set_flag(Flags::CmpEqual, self.reg_a8 == self.reg_c8);
                    self.set_flag(Flags::CmpLess, self.reg_a8 < self.reg_c8);
                },
                Instruction::CMPBC => {
                    self.set_flag(Flags::CmpGreater, self.reg_b8 > self.reg_c8);
                    self.set_flag(Flags::CmpEqual, self.reg_b8 == self.reg_c8);
                    self.set_flag(Flags::CmpLess, self.reg_b8 < self.reg_c8);
                },

                Instruction::DUMP => {
                    println!("{self}");
                }
            }
        }
    }
}