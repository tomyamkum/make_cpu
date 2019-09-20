struct Dff {
	pub pre_value: bool,
}

impl Dff {
	pub fn new(initial_state: bool) -> Dff {
		Dff {
			pre_value: initial_state,
		}
	}
	pub fn dff(&mut self, x: bool) -> bool {
		let result = self.pre_value;
		self.pre_value = x;
		result
	}
}
