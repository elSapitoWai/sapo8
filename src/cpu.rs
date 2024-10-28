pub struct Cpu {
    pub reg_file: [u8; 16],
    program_counter: usize,
    instructions: [u16; 32],
}

impl Cpu {
    // Clock
    pub fn run(&mut self) {
        loop {
            let instruction = self.instructions[self.program_counter];

            let opcode = instruction >> 12;

            let read1 = (instruction & 0b0000_1111_0000_0000) >> 8;
            let read2= (instruction & 0b0000_0000_1111_0000) >> 4;
            let write= instruction & 0b0000_0000_0000_1111;

            match opcode {
                // LDI
                0b1000 => {
                    self.reg_file[read1 as usize] = (instruction & 0b0000_0000_1111_1111) as u8;
                }

                // ADD
                0b0010 => {
                    self.reg_file[write as usize] = self.reg_file[read1 as usize].wrapping_add(self.reg_file[read2 as usize]);
                }

                // SUB
                0b0011 => {
                    self.reg_file[write as usize] = self.reg_file[read1 as usize].wrapping_sub(self.reg_file[read2 as usize]);
                }

                // NOR
                0b0100 => {
                    self.reg_file[write as usize] = !(self.reg_file[read1 as usize] | self.reg_file[read2 as usize]);
                }

                // AND
                0b0101 => {
                    self.reg_file[write as usize] = self.reg_file[read1 as usize] & self.reg_file[read2 as usize];
                }

                // XOR
                0b0110 => {
                    self.reg_file[write as usize] = self.reg_file[read1 as usize] ^ self.reg_file[read2 as usize];
                }

                _ => return,
            }

            if self.program_counter == 32 {
                return;
            }

            self.program_counter += 1;
        }
    }

    pub fn new(instructions: [u16; 32]) -> Cpu {
        Cpu {
            reg_file: [0; 16],
            program_counter: 0,
            instructions,
        }
    }
}