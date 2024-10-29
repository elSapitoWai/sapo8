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

            let read1 = ((instruction & 0b0000_1111_0000_0000) >> 8) as usize;
            let read2= ((instruction & 0b0000_0000_1111_0000) >> 4) as usize;
            let write= (instruction & 0b0000_0000_0000_1111) as usize;

            match opcode {
                // LDI
                0b1000 => {
                    self.reg_file[read1] = (instruction & 0b0000_0000_1111_1111) as u8;
                }

                // ADD
                0b0010 => {
                    self.reg_file[write] = self.reg_file[read1].wrapping_add(self.reg_file[read2]);
                }

                // SUB
                0b0011 => {
                    self.reg_file[write] = self.reg_file[read1].wrapping_sub(self.reg_file[read2]);
                }

                // NOR
                0b0100 => {
                    self.reg_file[write] = !(self.reg_file[read1] | self.reg_file[read2]);
                }

                // AND
                0b0101 => {
                    self.reg_file[write] = self.reg_file[read1] & self.reg_file[read2];
                }

                // XOR
                0b0110 => {
                    self.reg_file[write] = self.reg_file[read1] ^ self.reg_file[read2];
                }

                // RSH
                0b0111 => {
                    self.reg_file[write] = self.reg_file[read1] >> 1;
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