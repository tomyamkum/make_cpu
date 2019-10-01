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
		let c_inst = inst[15];
		let a_m_input = mux16(self.a_reg.load([true; 16], false), in_m, inst[12]);
		let d_input = self.d_reg.load([true; 16], false);
		let mut alu_result = ALUResult {
			out: [true; 16],
			zr: true, 
			ng: true,
		};

		alu_result = alu(d_input, a_m_input, inst[11], inst[10], inst[9], inst[8], inst[7], inst[6]);

		let dest = [
			and(c_inst, inst[5]),
			and(c_inst, inst[4]),
			and(c_inst, inst[3]),
		];

		let write_m = and(c_inst, inst[3]);

		let jump = [
			and(c_inst, inst[2]),
			and(c_inst, inst[1]),
			and(c_inst, inst[0]),
		];

		let out_is_pos = and(not(alu_result.zr), not(alu_result.ng));
		let out_is_zero = alu_result.zr;
		let out_is_neg = alu_result.ng;

		let pc_load = or(or(and(jump[2], out_is_pos), and(jump[1], out_is_zero)), and(jump[0], out_is_neg));

		self.d_reg.load(alu_result.out, dest[1]);
		let in_a = mux16(inst, alu_result.out, c_inst);
		self.a_reg.load(in_a, or(not(c_inst), dest[0]));
		let address_m = self.a_reg.load([true; 16], false);
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
			out_memory: alu_result.out,
			write_memory: write_m,
			address_memory: result_address_m,
			pc: result_pc,
		}
	}
}
