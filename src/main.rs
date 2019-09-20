extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let initial_value = vec![true,true,true,false,false,true,false,true,true,false,false,true,true,true,false,false];
	let next1 = vec![true,true,false,false,false,true,false,true,true,false,false,true,true,true,false,false];
	let next2 = vec![false,true,true,false,false,true,false,true,true,false,false,true,true,true,false,false];
	let next3 = vec![true,false,true,false,false,true,false,true,true,false,false,true,true,true,false,false];
	let mut reg16 = Reg16::new(initial_value);
	let value = reg16.get();
	println!("{:?}",value);
	reg16.next(&next1, true);
	let value = reg16.get();
	println!("{:?}",value);
	reg16.next(&next2, false);
	let value = reg16.get();
	println!("{:?}",value);
	reg16.next(&next3, true);
	let value = reg16.get();
	println!("{:?}",value);
}
