use crate::gate::*;
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
	pub bit: [Bit; 16],
}

impl Reg {
	pub fn new(initial_state: [bool; 16]) -> Reg {
		Reg {
			bit: [Bit::new(initial_state[0]), Bit::new(initial_state[1]), Bit::new(initial_state[2]), Bit::new(initial_state[3]), Bit::new(initial_state[4]), Bit::new(initial_state[5]), Bit::new(initial_state[6]), Bit::new(initial_state[7]), Bit::new(initial_state[8]), Bit::new(initial_state[9]), Bit::new(initial_state[10]), Bit::new(initial_state[11]), Bit::new(initial_state[12]), Bit::new(initial_state[13]), Bit::new(initial_state[14]), Bit::new(initial_state[15])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], load: bool) -> [bool; 16] {
		self.bit[0].dff.state = mux(self.bit[0].dff.state, input[0], load);
		self.bit[1].dff.state = mux(self.bit[1].dff.state, input[1], load);
		self.bit[2].dff.state = mux(self.bit[2].dff.state, input[2], load);
		self.bit[3].dff.state = mux(self.bit[3].dff.state, input[3], load);
		self.bit[4].dff.state = mux(self.bit[4].dff.state, input[4], load);
		self.bit[5].dff.state = mux(self.bit[5].dff.state, input[5], load);
		self.bit[6].dff.state = mux(self.bit[6].dff.state, input[6], load);
		self.bit[7].dff.state = mux(self.bit[7].dff.state, input[7], load);
		self.bit[8].dff.state = mux(self.bit[8].dff.state, input[8], load);
		self.bit[9].dff.state = mux(self.bit[9].dff.state, input[9], load);
		self.bit[10].dff.state = mux(self.bit[10].dff.state, input[10], load);
		self.bit[11].dff.state = mux(self.bit[11].dff.state, input[11], load);
		self.bit[12].dff.state = mux(self.bit[12].dff.state, input[12], load);
		self.bit[13].dff.state = mux(self.bit[13].dff.state, input[13], load);
		self.bit[14].dff.state = mux(self.bit[14].dff.state, input[14], load);
		self.bit[15].dff.state = mux(self.bit[15].dff.state, input[15], load);

		let mut ans: [bool; 16] = [true; 16];
		ans[0] = self.bit[0].dff.state;
		ans[1] = self.bit[1].dff.state;
		ans[2] = self.bit[2].dff.state;
		ans[3] = self.bit[3].dff.state;
		ans[4] = self.bit[4].dff.state;
		ans[5] = self.bit[5].dff.state;
		ans[6] = self.bit[6].dff.state;
		ans[7] = self.bit[7].dff.state;
		ans[8] = self.bit[8].dff.state;
		ans[9] = self.bit[9].dff.state;
		ans[10] = self.bit[10].dff.state;
		ans[11] = self.bit[11].dff.state;
		ans[12] = self.bit[12].dff.state;
		ans[13] = self.bit[13].dff.state;
		ans[14] = self.bit[14].dff.state;
		ans[15] = self.bit[15].dff.state;
		ans
	}
}

// 理解したこと
// new: boolの2次元配列を与えて,Reg配列を作り入れていく

// アドレスの指定方法!!!
pub struct Ram8 {
	pub reg: [Reg; 8],
}

impl Ram8 {
	pub fn new(initial_state: [[bool; 16]; 8]) -> Ram8 {
		Ram8 {
			reg: [Reg::new(initial_state[0]), Reg::new(initial_state[1]), Reg::new(initial_state[2]), Reg::new(initial_state[3]), Reg::new(initial_state[4]), Reg::new(initial_state[5]), Reg::new(initial_state[6]), Reg::new(initial_state[7])],
		}
	}
	pub fn load(&mut self, input: [bool; 16], address: usize, load: bool) -> [[bool; 16]; 8] {
		let tmpinput = [true; 16];
		self.reg[address] = Reg::new(mux16(self.reg[address].load(tmpinput, false), input, load));
		
		let mut ans: [[bool; 16]; 8] = [[true; 16]; 8];
		ans[0] = self.reg[0].load(tmpinput, false);
		ans[1] = self.reg[1].load(tmpinput, false);
		ans[2] = self.reg[2].load(tmpinput, false);
		ans[3] = self.reg[3].load(tmpinput, false);
		ans[4] = self.reg[4].load(tmpinput, false);
		ans[5] = self.reg[5].load(tmpinput, false);
		ans[6] = self.reg[6].load(tmpinput, false);
		ans[7] = self.reg[7].load(tmpinput, false);
		ans
	}
}

//pub struct Ram64 {
//	pub ram: Vec<Ram8>,
//}
//
//impl Ram64 {
//	pub fn new(initial_state: &Vec<Vec<bool>>) -> Ram64 {
//		Ram64 {
//			ram: vec![Ram8::new(initial_state[0]), Ram8::new(initial_state[1]), Ram8::new(initial_state[2]), Ram8::new(initial_state[3]), Ram8::new(initial_state[4]), Ram8::new(initial_state[5]), Ram8::new(initial_state[6]), Ram8::new(initial_state[7])],
//		}
//	}
//	pub fn load(&mut self, input: &Vec<bool>
//}
