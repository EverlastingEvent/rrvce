use crate::cpu::CPU;
use crate::instruction::Instruction;

struct Emulator {
    cpu: CPU,
    instructions: Vec<Instruction>
}
