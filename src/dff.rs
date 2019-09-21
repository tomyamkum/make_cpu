pub struct Dff {
	pub state: bool,
}

impl Dff {
	pub fn new(initial_state: bool) -> Dff {
		Dff {
			state: initial_state,
		}
	}
	pub fn load(&mut self, x: bool) -> bool {
		let result = self.state;
		self.state = x;
		result
	}
}
