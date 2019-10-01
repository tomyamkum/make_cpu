use make_cpu::computer::*;

fn main() {
	let mut computer = Computer::new();
	computer.load("test.txt");
	computer.exec(false);
}
