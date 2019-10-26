use make_cpu::computer::*;
use make_cpu::assembler::*;

fn main() {
	let mut computer = Computer::new();
	computer.load("test.hack");
	computer.exec(false);
	//assemble("test.asm");
}
