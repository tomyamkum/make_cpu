use make_cpu::comb_circ::*;
use make_cpu::seq_circ::*;

fn main() {
	let mut pc = PC::new();
	let input = [true, true, false, false, false, false, false, true, false, false, false, false, false, true, true, false];
	let inc = true;
	let load = false;
	let reset = false;
	println!("{:?}", pc.next(input, inc, load, reset));
	println!("{:?}", pc.next(input, inc, load, reset));
	println!("{:?}", pc.next(input, inc, load, reset));
	println!("{:?}", pc.next(input, inc, load, reset));
}
