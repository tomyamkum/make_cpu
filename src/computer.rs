use std::fs;

use crate::cpu::*;
use crate::seq_circ::*;
use crate::util::*;

pub struct Computer {
	pub rom: Ram32K,
	pub memory: Ram32K,
	pub screen: Ram8K,
	pub cpu: CPU,
}

impl Computer {
	pub fn new() -> Computer {
		Computer {
			rom: Ram32K::new([[[[[[false; 16]; 8]; 8]; 8]; 8]; 8]),
			memory: Ram32K::new([[[[[[false; 16]; 8]; 8]; 8]; 8]; 8]),
			screen: Ram8K::new([[[[[[false; 16]; 8]; 8]; 8]; 8]; 2]),
			cpu: CPU::new(),
		}
	}
	pub fn load(&mut self, path: &str) {
		let content = fs::read_to_string(path).unwrap();
		let mut inst = vec![];
		let mut tmp: [bool; 16] = [true; 16];
		let mut index = 0;
		for i in content.as_str().chars() {
			if i == '\n' {
				inst.push(tmp);
				index = 0;
			}
			else {
				if i == '1' {
					tmp[15-index] = true;
					index += 1;
				}
				else {
					tmp[15-index] = false;
					index += 1;
				}
			}
		}
		for i in 0..inst.len() {
			let inst_tmp = inst[i];
			self.rom.load(inst_tmp, u2b15(i as u16), true);
		}
	}

	pub fn exec(&mut self, reset: bool) -> [bool; 16] {
		let mut cpu_result = CPUResult {
			out_memory: [true; 16],
			write_memory: true,
			address_memory: [true; 15],
			pc: [false; 15],
		};
		let mut in_m = [false; 16];
		loop {
			let inst = self.rom.load([true; 16], cpu_result.pc, false);
			cpu_result = self.cpu.next(inst, in_m, reset);
			in_m = self.memory.load(cpu_result.out_memory, cpu_result.address_memory, cpu_result.write_memory);
			println!("{:?}", self.memory.load([true; 16], [true,false,false,false,true,false,false,false,false,false,false,false,false,false,false], false));
		}
		self.memory.load([false; 16], [false; 15], false)
	}
}
