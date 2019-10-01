use make_cpu::computer::*;
use make_cpu::seq_circ::*;

fn main() {
	let mut computer = Computer::new();
	computer.load("test.txt");
	println!("{:?}",computer.exec(false));
}
