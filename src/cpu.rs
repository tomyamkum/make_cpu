use crate::gate::*;
use crate::comb_circ::*;
use crate::seq_circ::*;

pub struct CPU {
	pub pc: PC,
	pub a_reg: Reg,
	pub d_reg: Reg,
}

pub struct CPUResult {
	pub out_memory: [bool; 16],
	pub write_memory: bool, 
	pub address_memory: [bool; 15],
	pub pc: [bool; 15],
}

impl CPU {
	pub fn new() -> CPU {
		CPU {
			pc: PC::new(),
			a_reg: Reg::new([false; 16]),
			d_reg: Reg::new([false; 16]),
		}
	}

	pub fn next(&mut self, inst: [bool; 16], in_m: [bool; 16], reset: bool) -> CPUResult {
		let c_inst = inst[0];
		let a_m_input = mux16(self.a_reg.load([true; 16], false), in_m, inst[3]);
		let address_m = self.a_reg.load([true; 16], false);
		let d_input = self.d_reg.load([true; 16], false);

		let (out_m, c1, c2) = alu(d_input, a_m_input, inst[4], inst[5], inst[6], inst[7], inst[8], inst[9]);
		let write_m = inst[12];

		let dest = [
			and(c_inst, inst[10]),
			and(c_inst, inst[11]),
			and(c_inst, inst[12]),
		];

		let jump = [
			and(c_inst, inst[13]),
			and(c_inst, inst[14]),
			and(c_inst, inst[15]),
		];

		let out_is_pos = and(not(c1), not(c2));
		let out_is_zero = c1;
		let out_is_neg = c2;

		let pc_load = or(or(and(jump[2], out_is_pos), and(jump[1], out_is_zero)), and(jump[0], out_is_neg));

		self.d_reg.load(out_m, dest[1]);
		let in_a = mux16(inst, out_m, c_inst);
		self.a_reg.load(in_a, or(not(c_inst), dest[0]));
		let pc_out = self.pc.next(address_m, true, pc_load, reset);
		let result_address_m = [
			address_m[0],
			address_m[1],
			address_m[2],
			address_m[3],
			address_m[4],
			address_m[5],
			address_m[6],
			address_m[7],
			address_m[8],
			address_m[9],
			address_m[10],
			address_m[11],
			address_m[12],
			address_m[13],
			address_m[14],
		];
		let result_pc = [
			pc_out[0],
			pc_out[1],
			pc_out[2],
			pc_out[3],
			pc_out[4],
			pc_out[5],
			pc_out[6],
			pc_out[7],
			pc_out[8],
			pc_out[9],
			pc_out[10],
			pc_out[11],
			pc_out[12],
			pc_out[13],
			pc_out[14],
		];
		CPUResult {
			out_memory: out_m,
			write_memory: write_m,
			address_memory: result_address_m,
			pc: result_pc,
		}
	}
}
