use make_cpu::cpu::*;
use make_cpu::computer::*;
use make_cpu::gate::*;
use make_cpu::comb_circ::*;
use make_cpu::util::*;

fn main() {
	let mut cpu = CPU::new();
	let mut inst = [true; 16];
	inst[0] = false;
	let in_m = [true; 16];
	let reset = false;
	let (out_m, write_m, address_m, pc) = cpu.next(inst, in_m, reset);
	let (out_m, write_m, address_m, pc) = cpu.next(inst, in_m, reset);
	let (out_m, write_m, address_m, pc) = cpu.next(inst, in_m, reset);
	let (out_m, write_m, address_m, pc) = cpu.next(inst, in_m, reset);
	println!("{:?}", pc);

	let path = "test.txt";
	let mut computer = Computer::new();
	computer.load("test.txt");
	println!("{:?}", computer.rom.load([true; 16], [true; 15], false));

}
