
/// CPU structure of the UM containing registers
pub struct CPU {
    pub registers: Vec<u32>
}

impl CPU {
    // CPU constructor with eight registers of u32's
    pub fn new() -> Self {
        CPU {
            registers: vec![0_u32; 8]
        }
    }
    // write a value on a register
    pub fn write(&mut self, val: u32, register: u32) {
        self.registers[register as usize] = val
    }
    // return a value from a register 
    pub fn read(&self, register: u32) -> u32 {
        self.registers[register as usize]
    }
}