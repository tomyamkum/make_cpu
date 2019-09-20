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
	pub fn next(&mut self, input: bool, load: bool) -> bool {
		self.dff.pre_value = mux(self.dff.pre_value, input, load);
		self.dff.pre_value
	}
}
