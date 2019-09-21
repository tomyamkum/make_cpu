extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let mut input = [[true; 16]; 8];
	let mut ram = Ram8::new(input);
	let next = [false; 16];
	println!("{:?}", ram.load(next, 0, true));
}
