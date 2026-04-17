#[derive(Debug, Clone)]
pub enum Instruction {
    Add {rd: usize, rs1: usize, rs2: usize },
    // TODO!
    Nop,
}
