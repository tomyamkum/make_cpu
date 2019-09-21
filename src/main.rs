extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let mut input = Vec::new();
	input.push(vec![true,true,false,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![false,true,true,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![true,false,true,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![true,true,false,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![false,true,true,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![true,false,true,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![true,true,false,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	input.push(vec![false,true,true,false,false,true,false,true,true,false,false,true,true,true,false,false]);
	let mut ram = Ram8::new(&input);
	let next = vec![true,true,true,false,true,false,false,true,true,true,true,true,true,true,true,true];
	println!("{:?}",ram.load(&next, 0, true));
}
