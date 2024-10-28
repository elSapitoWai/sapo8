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
        assert_eq!(cpu.reg_file[3], 5);
    }
}