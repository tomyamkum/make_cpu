use crate::gate::*;

pub fn mux(x: bool, y: bool, sel: bool) -> bool {
	or(and(x, not(sel)), or(and(x, y), and(y, sel)))
}

pub fn mux16(x: [bool; 16], y: [bool; 16], sel: bool) -> [bool; 16] {
	[
		mux(x[0], y[0], sel),
		mux(x[1], y[1], sel),
		mux(x[2], y[2], sel),
		mux(x[3], y[3], sel),
		mux(x[4], y[4], sel),
		mux(x[5], y[5], sel),
		mux(x[6], y[6], sel),
		mux(x[7], y[7], sel),
		mux(x[8], y[8], sel),
		mux(x[9], y[9], sel),
		mux(x[10], y[10], sel),
		mux(x[11], y[11], sel),
		mux(x[12], y[12], sel),
		mux(x[13], y[13], sel),
		mux(x[14], y[14], sel),
		mux(x[15], y[15], sel),
	]
}

pub fn mux4way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], sel: [bool; 2]) -> [bool; 16] {
	let ans = mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1]);
	ans
}

pub fn mux8way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], e: [bool; 16], f: [bool; 16], g: [bool; 16], h: [bool; 16], sel: [bool; 3]) -> [bool; 16] {
	let mut tmpsel = [true, true];
	tmpsel[0] = sel[0];
	tmpsel[1] = sel[1];
	let ans = mux16(mux4way16(a, b, c, d, tmpsel), mux4way16(e, f, g, h, tmpsel), sel[2]);
	ans
}

pub fn dmux(input: bool, sel: bool) -> [bool; 2] {
	let mut ans: [bool; 2] = [true, true];
	ans[0] = and(input, not(sel));
	ans[1] = and(input, sel);
	ans
}

pub fn dmux4way(input: bool, sel: [bool; 2]) -> [bool; 4] {
	let ans = [dmux(dmux(input, sel[1])[0], sel[0])[0], dmux(dmux(input, sel[1])[0], sel[0])[1], dmux(dmux(input, sel[1])[1], sel[0])[0], dmux(dmux(input, sel[1])[1], sel[0])[1]];
	ans
}

pub fn dmux8way(input: bool, sel: [bool; 3]) -> [bool; 8] {
	let mut tmpsel = [true, true];
	tmpsel[0] = sel[0];
	tmpsel[1] = sel[1];
	let tmpans = dmux4way(input, tmpsel);
	let mut ans: [bool; 8] = [true; 8];
	ans[0] = dmux(tmpans[0], sel[2])[0];
	ans[4] = dmux(tmpans[0], sel[2])[1];
	ans[1] = dmux(tmpans[1], sel[2])[0];
	ans[5] = dmux(tmpans[1], sel[2])[1];
	ans[2] = dmux(tmpans[2], sel[2])[0];
	ans[6] = dmux(tmpans[2], sel[2])[1];
	ans[3] = dmux(tmpans[3], sel[2])[0];
	ans[7] = dmux(tmpans[3], sel[2])[1];
	ans	 
}

pub fn halfaddr(x: bool, y: bool) -> [bool; 2] {
	[and(x, y), xor(x, y)]
}

pub fn fulladdr(x: bool, y: bool, c: bool) -> [bool; 2] {
	let [c1, s1] = halfaddr(x, y);
	let [c2, s2] = halfaddr(s1, c);
	[or(c1, c2), s2]
}

pub fn add16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	let mut tmp: [bool; 2];
	let mut c = false;
	tmp = fulladdr(x[0], y[0], c);
	ans[0] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[1], y[1], c);
	ans[1] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[2], y[2], c);
	ans[2] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[3], y[3], c);
	ans[3] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[4], y[4], c);
	ans[4] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[5], y[5], c);
	ans[5] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[6], y[6], c);
	ans[6] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[7], y[7], c);
	ans[7] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[8], y[8], c);
	ans[8] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[9], y[9], c);
	ans[9] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[10], y[10], c);
	ans[10] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[11], y[11], c);
	ans[11] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[12], y[12], c);
	ans[12] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[13], y[13], c);
	ans[13] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[14], y[14], c);
	ans[14] = tmp[1];
	c = tmp[0];
	tmp = fulladdr(x[15], y[15], c);
	ans[15] = tmp[1];
	ans
}

pub fn inc16(x: [bool; 16]) -> [bool; 16]  {
	let y = [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
	add16(x, y)
}

pub struct ALUResult {
	pub out: [bool; 16],
	pub zr: bool,
	pub ng: bool,
}

pub fn alu(x: [bool; 16], y: [bool; 16], zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> ALUResult {
	let x = and16_1(x, not(zx));

	let x = xor16_1(x, nx);

	let y = and16_1(y, not(zy));

	let y = xor16_1(y, ny);

	let tmp1 = add16(x, y);
	let tmp2 = and16_16(x, y);
	let tmp3 = mux16(tmp2, tmp1, f);
	let ans = xor16_1(tmp3, no);
	let zr = not(or16way(ans));
	let ng = ans[15];
	ALUResult {
		out: ans,
		zr: zr,
		ng: ng,
	}
}
