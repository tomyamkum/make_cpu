extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let mut reg = Reg::new(false);
	println!("{}", reg.next(true, true));
	println!("{}", reg.next(true, false));
	println!("{}", reg.next(false, true));
	println!("{}", reg.next(true, false));
	println!("{}", reg.next(true, true));
}
