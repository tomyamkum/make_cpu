use crate::gate::*;
use crate::dff::*;
use crate::comb_circ::*;

pub struct Reg {
	pub dff: Dff,
}

impl Reg {
	pub fn new(initial_value: bool) -> Reg {
		Reg{
			 dff: Dff::new(initial_value),
		}
	}
	pub fn next(&mut self, input: bool, load: bool) {
		self.dff.pre_value = mux(self.dff.pre_value, input, load);
	}
	pub fn get(&self) -> bool {
		self.dff.pre_value
	}
}

pub struct Reg16 {
	pub reg: Vec<Reg>,
}

impl Reg16 {
	pub fn new(initial_value: Vec<bool>) -> Reg16 {
		Reg16 {
			reg: vec![Reg::new(initial_value[0]), Reg::new(initial_value[1]), Reg::new(initial_value[2]), Reg::new(initial_value[3]), Reg::new(initial_value[4]), Reg::new(initial_value[5]), Reg::new(initial_value[6]), Reg::new(initial_value[7]), Reg::new(initial_value[8]), Reg::new(initial_value[9]), Reg::new(initial_value[10]), Reg::new(initial_value[11]), Reg::new(initial_value[12]), Reg::new(initial_value[13]), Reg::new(initial_value[14]), Reg::new(initial_value[15])],
		}
	}
	pub fn next(&mut self, input: &Vec<bool>, load: bool) {
		self.reg[0].dff.pre_value = mux(self.reg[0].dff.pre_value, input[0], load);
		self.reg[1].dff.pre_value = mux(self.reg[1].dff.pre_value, input[1], load);
		self.reg[2].dff.pre_value = mux(self.reg[2].dff.pre_value, input[2], load);
		self.reg[3].dff.pre_value = mux(self.reg[3].dff.pre_value, input[3], load);
		self.reg[4].dff.pre_value = mux(self.reg[4].dff.pre_value, input[4], load);
		self.reg[5].dff.pre_value = mux(self.reg[5].dff.pre_value, input[5], load);
		self.reg[6].dff.pre_value = mux(self.reg[6].dff.pre_value, input[6], load);
		self.reg[7].dff.pre_value = mux(self.reg[7].dff.pre_value, input[7], load);
		self.reg[8].dff.pre_value = mux(self.reg[8].dff.pre_value, input[8], load);
		self.reg[9].dff.pre_value = mux(self.reg[9].dff.pre_value, input[9], load);
		self.reg[10].dff.pre_value = mux(self.reg[10].dff.pre_value, input[10], load);
		self.reg[11].dff.pre_value = mux(self.reg[11].dff.pre_value, input[11], load);
		self.reg[12].dff.pre_value = mux(self.reg[12].dff.pre_value, input[12], load);
		self.reg[13].dff.pre_value = mux(self.reg[13].dff.pre_value, input[13], load);
		self.reg[14].dff.pre_value = mux(self.reg[14].dff.pre_value, input[14], load);
		self.reg[15].dff.pre_value = mux(self.reg[15].dff.pre_value, input[15], load);
	}
	pub fn get(&self) -> Vec<bool> {
		let mut ans = Vec::new();
		ans.push(self.reg[0].dff.pre_value);
		ans.push(self.reg[1].dff.pre_value);
		ans.push(self.reg[2].dff.pre_value);
		ans.push(self.reg[3].dff.pre_value);
		ans.push(self.reg[4].dff.pre_value);
		ans.push(self.reg[5].dff.pre_value);
		ans.push(self.reg[6].dff.pre_value);
		ans.push(self.reg[7].dff.pre_value);
		ans.push(self.reg[8].dff.pre_value);
		ans.push(self.reg[9].dff.pre_value);
		ans.push(self.reg[10].dff.pre_value);
		ans.push(self.reg[11].dff.pre_value);
		ans.push(self.reg[12].dff.pre_value);
		ans.push(self.reg[13].dff.pre_value);
		ans.push(self.reg[14].dff.pre_value);
		ans.push(self.reg[15].dff.pre_value);
		ans
	}
}
