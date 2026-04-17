pub struct CPU {
    registers: [i32; 32],   // Целочисленные регисты x0-x31
    f_registers: [f32; 32], // Регистры с плавающей точкой f0-f31
    pc: usize,              // Программный счётчик
    memory: Vec<u8>         // Память 
}