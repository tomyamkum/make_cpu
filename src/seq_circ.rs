use crate::dff::*;
use crate::comb_circ::*;

pub struct Bit {
	pub dff: Dff,
}

impl Bit {
	pub fn new(initial_state: bool) -> Bit {
		Bit {
			 dff: Dff::new(initial_state),
		}
	}
	pub fn load(&mut self, input: bool, load: bool) -> bool {
		self.dff.state = mux(self.dff.state, input, load);
		let output = self.dff.state;
		output
	}
}

pub struct Reg {
	pub bits: [Bit; 16],
}

impl Reg {
	pub fn new(initial_state: [bool; 16]) -> Reg {
		Reg {
			bits: [Bit::new(initial_state[0]), Bit::new(initial_state[1]), Bit::new(initial_state[2]), Bit::new(initial_state[3]), Bit::new(initial_state[4]), Bit::new(initial_state[5]), Bit::new(initial_state[6]), Bit::new(initial_state[7]), Bit::new(initial_state[8]), Bit::new(initial_state[9]), Bit::new(initial_state[10]), Bit::new(initial_state[11]), Bit::new(initial_state[12]), Bit::new(initial_state[13]), Bit::new(initial_state[14]), Bit::new(initial_state[15])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], load: bool) -> [bool; 16] {
		[
			self.bits[0].load(input[0], load),
			self.bits[1].load(input[1], load),
			self.bits[2].load(input[2], load),
			self.bits[3].load(input[3], load),
			self.bits[4].load(input[4], load),
			self.bits[5].load(input[5], load),
			self.bits[6].load(input[6], load),
			self.bits[7].load(input[7], load),
			self.bits[8].load(input[8], load),
			self.bits[9].load(input[9], load),
			self.bits[10].load(input[10], load),
			self.bits[11].load(input[11], load),
			self.bits[12].load(input[12], load),
			self.bits[13].load(input[13], load),
			self.bits[14].load(input[14], load),
			self.bits[15].load(input[15], load),
		]
	}
}

pub struct Ram8 {
	pub regs: [Reg; 8],
}

impl Ram8 {
	pub fn new(initial_state: [[bool; 16]; 8]) -> Ram8 {
		Ram8 {
			regs: [Reg::new(initial_state[0]), Reg::new(initial_state[1]), Reg::new(initial_state[2]), Reg::new(initial_state[3]), Reg::new(initial_state[4]), Reg::new(initial_state[5]), Reg::new(initial_state[6]), Reg::new(initial_state[7])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 3], load: bool) -> [bool; 16] {
		let sel = dmux8way(load, address);
		mux8way16(
			self.regs[0].load(input, sel[0]),
			self.regs[1].load(input, sel[1]),
			self.regs[2].load(input, sel[2]),
			self.regs[3].load(input, sel[3]),
			self.regs[4].load(input, sel[4]),
			self.regs[5].load(input, sel[5]),
			self.regs[6].load(input, sel[6]),
			self.regs[7].load(input, sel[7]),
			address,
		)
	}
}

pub struct Ram64 {
	pub ram8s: [Ram8; 8],
}

impl Ram64 {
	pub fn new(initial_state: [[[bool; 16]; 8]; 8]) -> Ram64 {
		Ram64 {
			ram8s: [Ram8::new(initial_state[0]), Ram8::new(initial_state[1]), Ram8::new(initial_state[2]), Ram8::new(initial_state[3]), Ram8::new(initial_state[4]), Ram8::new(initial_state[5]), Ram8::new(initial_state[6]), Ram8::new(initial_state[7])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 6], load: bool) -> [bool; 16] {
		let address1 = [address[0], address[1], address[2]];
		let address2 = [address[3], address[4], address[5]];
		let sel = dmux8way(load, address1);

		mux8way16(
			self.ram8s[0].load(input, address2, sel[0]),
			self.ram8s[1].load(input, address2, sel[1]),
			self.ram8s[2].load(input, address2, sel[2]),
			self.ram8s[3].load(input, address2, sel[3]),
			self.ram8s[4].load(input, address2, sel[4]),
			self.ram8s[5].load(input, address2, sel[5]),
			self.ram8s[6].load(input, address2, sel[6]),
			self.ram8s[7].load(input, address2, sel[7]),
			address1,
		)
	}
}

pub struct Ram512 {
	pub ram64s: [Ram64; 8],
}

impl Ram512 {
	pub fn new(initial_state: [[[[bool; 16]; 8]; 8]; 8]) -> Ram512 {
		Ram512 {
			ram64s: [Ram64::new(initial_state[0]), Ram64::new(initial_state[1]), Ram64::new(initial_state[2]), Ram64::new(initial_state[3]), Ram64::new(initial_state[4]), Ram64::new(initial_state[5]), Ram64::new(initial_state[6]), Ram64::new(initial_state[7])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 9], load: bool) -> [bool; 16] {
		let address1 = [address[0], address[1], address[2]];
		let address2 = [address[3], address[4], address[5], address[6], address[7], address[8]];
		let sel = dmux8way(load, address1);

		mux8way16(
			self.ram64s[0].load(input, address2, sel[0]),
			self.ram64s[1].load(input, address2, sel[1]),
			self.ram64s[2].load(input, address2, sel[2]),
			self.ram64s[3].load(input, address2, sel[3]),
			self.ram64s[4].load(input, address2, sel[4]),
			self.ram64s[5].load(input, address2, sel[5]),
			self.ram64s[6].load(input, address2, sel[6]),
			self.ram64s[7].load(input, address2, sel[7]),
			address1,
		)
	}
}

pub struct Ram4K {
	pub ram512s: [Ram512; 8],
}

impl Ram4K {
	pub fn new(initial_state: [[[[[bool; 16]; 8]; 8]; 8]; 8]) -> Ram4K {
		Ram4K {
			ram512s: [Ram512::new(initial_state[0]), Ram512::new(initial_state[1]), Ram512::new(initial_state[2]), Ram512::new(initial_state[3]), Ram512::new(initial_state[4]), Ram512::new(initial_state[5]), Ram512::new(initial_state[6]), Ram512::new(initial_state[7])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 12], load: bool) -> [bool; 16] {
		let address1 = [address[0], address[1], address[2]];
		let address2 = [address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11]];
		let sel = dmux8way(load, address1);

		mux8way16(
			self.ram512s[0].load(input, address2, sel[0]),
			self.ram512s[1].load(input, address2, sel[1]),
			self.ram512s[2].load(input, address2, sel[2]),
			self.ram512s[3].load(input, address2, sel[3]),
			self.ram512s[4].load(input, address2, sel[4]),
			self.ram512s[5].load(input, address2, sel[5]),
			self.ram512s[6].load(input, address2, sel[6]),
			self.ram512s[7].load(input, address2, sel[7]),
			address1,
		)
	}
}

pub struct Ram8K {
	pub ram4ks: [Ram4K; 2],
}

impl Ram8K {
	pub fn new(initial_state: [[[[[[bool; 16]; 8]; 8]; 8]; 8]; 2]) -> Ram8K {
		Ram8K {
			ram4ks: [Ram4K::new(initial_state[0]), Ram4K::new(initial_state[1])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 13], load: bool) -> [bool; 16] {
		let address1 = address[0];
		let address2 = [address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11], address[12]];
		let sel = dmux(load, address1);

		mux16(
			self.ram4ks[0].load(input, address2, sel[0]),
			self.ram4ks[1].load(input, address2, sel[1]),
			address1,
		)
	}
}

pub struct Ram16K {
	pub ram4ks: [Ram4K; 4],
}

impl Ram16K {
	pub fn new(initial_state: [[[[[[bool; 16]; 8]; 8]; 8]; 8]; 4]) -> Ram16K {
		Ram16K {
			ram4ks: [Ram4K::new(initial_state[0]), Ram4K::new(initial_state[1]), Ram4K::new(initial_state[2]), Ram4K::new(initial_state[3])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 14], load: bool) -> [bool; 16] {
		let address1 = [address[0], address[1]];
		let address2 = [address[2], address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11], address[12], address[13]];
		let sel = dmux4way(load, address1);

		mux4way16(
			self.ram4ks[0].load(input, address2, sel[0]),
			self.ram4ks[1].load(input, address2, sel[1]),
			self.ram4ks[2].load(input, address2, sel[2]),
			self.ram4ks[3].load(input, address2, sel[3]),
			address1,
		)
	}
}

pub struct Ram32K {
	pub ram4ks: [Ram4K; 8],
}

impl Ram32K {
	pub fn new(initial_state: [[[[[[bool; 16]; 8]; 8]; 8]; 8]; 8]) -> Ram32K {
		Ram32K {
			ram4ks: [Ram4K::new(initial_state[0]), Ram4K::new(initial_state[1]), Ram4K::new(initial_state[2]), Ram4K::new(initial_state[3]), Ram4K::new(initial_state[4]), Ram4K::new(initial_state[5]), Ram4K::new(initial_state[6]), Ram4K::new(initial_state[7])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: [bool; 15], load: bool) -> [bool; 16] {
		let address1 = [address[0], address[1], address[2]];
		let address2 = [address[3], address[4], address[5], address[6], address[7], address[8], address[9], address[10], address[11], address[12], address[13], address[14]];
		let sel = dmux8way(load, address1);

		mux8way16(
			self.ram4ks[0].load(input, address2, sel[0]),
			self.ram4ks[1].load(input, address2, sel[1]),
			self.ram4ks[2].load(input, address2, sel[2]),
			self.ram4ks[3].load(input, address2, sel[3]),
			self.ram4ks[4].load(input, address2, sel[4]),
			self.ram4ks[5].load(input, address2, sel[5]),
			self.ram4ks[6].load(input, address2, sel[6]),
			self.ram4ks[7].load(input, address2, sel[7]),
			address1,
		)
	}
}

pub struct PC {
	pub reg: Reg,
}

impl PC {
	pub fn new() -> PC {
		PC {
			reg: Reg::new([false; 16]),
		}
	}

	pub fn next(&mut self, input: [bool; 16], inc: bool, load: bool, reset: bool) -> [bool; 16] {
		let mut state = self.reg.load([true; 16], false);
		let mut _state = self.reg.load(inc16(state), inc);
		_state = self.reg.load(input, load);
		state = self.reg.load([false; 16], reset);
		[
			state[0],
			state[1],
			state[2],
			state[3],
			state[4],
			state[5],
			state[6],
			state[7],
			state[8],
			state[9],
			state[10],
			state[11],
			state[12],
			state[13],
			state[14],
			state[15],
		]
	}
}
