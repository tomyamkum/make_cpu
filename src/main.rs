extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let input = true;
	let sel = [true, true, true];
	println!("{:?}", dmux8way(input, sel));
}
