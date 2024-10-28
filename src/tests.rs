#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn add_two_integers() {
        let mut instructions = [0; 32];
        instructions[0] = 0b0111_0001_0000_0011;
        instructions[1] = 0b0111_0010_0000_0010;
        instructions[2] = 0b1101_0001_0010_0011;
        let mut cpu = Cpu::new(instructions);
        cpu.run();
        assert_eq!(cpu.reg_file[1], 3);
        assert_eq!(cpu.reg_file[2], 2);
        assert_eq!(cpu.reg_file[3], 5);
    }

    #[test]
    fn add_overflow() {
        let mut instructions = [0; 32];
        instructions[0] = 0b0111_0001_1111_1111;
        instructions[1] = 0b0111_0010_0000_0001;
        instructions[2] = 0b1101_0001_0010_0011;
        let mut cpu = Cpu::new(instructions);
        cpu.run();
        assert_eq!(cpu.reg_file[1], 0b1111_1111);
        assert_eq!(cpu.reg_file[2], 1);
        assert_eq!(cpu.reg_file[3], 0);
    }

    #[test]
    fn sub_two_integers() {
        let mut instructions = [0; 32];
        instructions[0] = 0b0111_0001_0000_0011;
        instructions[1] = 0b0111_0010_0000_0010;
        instructions[2] = 0b1100_0001_0010_0011;
        let mut cpu = Cpu::new(instructions);
        cpu.run();
        assert_eq!(cpu.reg_file[1], 3);
        assert_eq!(cpu.reg_file[2], 2);
        assert_eq!(cpu.reg_file[3], 1);
    }

    #[test]
    fn sub_overflow() {
        let mut instructions = [0; 32];
        instructions[0] = 0b0111_0001_0000_0000;
        instructions[1] = 0b0111_0010_0000_0001;
        instructions[2] = 0b1100_0001_0010_0011;
        let mut cpu = Cpu::new(instructions);
        cpu.run();
        assert_eq!(cpu.reg_file[1], 0);
        assert_eq!(cpu.reg_file[2], 1);
        assert_eq!(cpu.reg_file[3], 0b1111_1111);
    }
}