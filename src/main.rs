extern crate make_cpu;

use make_cpu::gate::*;
use make_cpu::circuit::*;

fn main() {
	let mut a: Vec<bool> = Vec::new();
	let mut b: Vec<bool> = Vec::new();
	a.push(true);
	a.push(false);
	a.push(true);
	a.push(true);
	a.push(false);
	a.push(true);
	a.push(false);
	a.push(true);
	a.push(true);
	a.push(true);
	a.push(true);
	a.push(true);
	a.push(true);
	a.push(true);
	a.push(true);
	a.push(true);
	b.push(true);
	b.push(false);
	b.push(true);
	b.push(true);
	b.push(false);
	b.push(true);
	b.push(false);
	b.push(true);
	b.push(true);
	b.push(true);
	b.push(true);
	b.push(true);
	b.push(true);
	b.push(true);
	b.push(true);
	b.push(true);
	let (ans,x,y) = alu(&mut a,&mut b, true,true,true,true,true,true);
	println!("{:?}", a);
	println!("{:?}", ans);
}
