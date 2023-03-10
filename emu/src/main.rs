use arch::Instruction;
use machine::Machine;

pub mod machine;
pub mod arch;

fn main() {
    let memory = Box::new([
        Instruction::INCA as u8,
        Instruction::INCA as u8,
        Instruction::INCA as u8,
        Instruction::INCA as u8,

        Instruction::INCB as u8,
        Instruction::INCB as u8,
        Instruction::INCB as u8,

        Instruction::SUBAB as u8,

        Instruction::DUMP as u8,
    ]);
    let mut machine = Machine::new(memory);
    machine.exec();
}
