mod cpu;
mod tests;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new([0; 32]);
    cpu.run();
    
    
}
