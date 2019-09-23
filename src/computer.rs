use std::fs;

use crate::cpu::*;
use crate::seq_circ::*;
use crate::util::*;

pub struct Computer {
	pub rom: Ram32K,
	pub memory: Ram16K,
	pub cpu: CPU,
}

impl Computer {
	pub fn new() -> Computer {
		Computer {
			rom: Ram32K::new([[[[[[false; 16]; 8]; 8]; 8]; 8]; 8]),
			memory: Ram16K::new([[[[[[false; 16]; 8]; 8]; 8]; 8]; 4]),
			cpu: CPU::new(),
		}
	}
	pub fn load(&mut self, path: &str) {
		let mut content = fs::read_to_string(path).unwrap();
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
					tmp[index] = true;
					index += 1;
				}
				else {
					tmp[index] = false;
					index += 1;
				}
			}
		}
		for i in 0..inst.len() {
			let inst_tmp = inst[i];
			self.rom.load(inst_tmp, u2b15(i as u16), true);
		}
	}
	pub fn exec(reset: bool) {
		let pc = 0; 
		loop {
			let inst = rom.load([true; 16], u2b15(pc as u16), false);
			if inst == [false; 16] {
				break;
			}
			let (out_m, write_m, address_m, pc) = cpu.next(inst, in_m, reset);
		}
	}
}
