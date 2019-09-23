use make_cpu::cpu::*;
use make_cpu::computer::*;
use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::util::*;

fn main() {
	let mut computer = Computer::new();
	computer.load("test.txt");
	println!("{:?}",computer.exec(false));
}
