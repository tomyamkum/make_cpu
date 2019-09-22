extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let initial_state = [[[[[[true; 16]; 8]; 8]; 8]; 8]; 4];
	let mut a = Ram16K::new(initial_state);
	let next_state = [false; 16];
	let address = [false; 14];
	println!("{:?}", a.load(next_state, address, true));
}
