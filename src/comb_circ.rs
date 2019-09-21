use crate::gate::*;

pub fn mux(x: bool, y: bool, sel: bool) -> bool {
	or(and(x, not(sel)), or(and(x, y), and(y, sel)))
}

pub fn mux16(x: [bool; 16], y: [bool; 16], sel: bool) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = mux(x[0], y[0], sel);
	ans[1] = mux(x[1], y[1], sel);
	ans[2] = mux(x[2], y[2], sel);
	ans[3] = mux(x[3], y[3], sel);
	ans[4] = mux(x[4], y[4], sel);
	ans[5] = mux(x[5], y[5], sel);
	ans[6] = mux(x[6], y[6], sel);
	ans[7] = mux(x[7], y[7], sel);
	ans[8] = mux(x[8], y[8], sel);
	ans[9] = mux(x[9], y[9], sel);
	ans[10] = mux(x[10], y[10], sel);
	ans[11] = mux(x[11], y[11], sel);
	ans[12] = mux(x[12], y[12], sel);
	ans[13] = mux(x[13], y[13], sel);
	ans[14] = mux(x[14], y[14], sel);
	ans[15] = mux(x[15], y[15], sel);
	ans
}

pub fn mux4way16(a: [bool; 16], b: [bool; 16], c: [bool; 16], d: [bool; 16], sel: [bool; 2]) -> [bool; 16] {
	let ans = mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1]);
	ans
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
	(and(input, not(sel)), and(input, sel))
}

pub fn halfaddr(x: bool, y: bool) -> (bool, bool) {
	(and(x, y), xor(x, y))
}

pub fn fulladdr(x: bool, y: bool, c: bool) -> (bool, bool) {
	let (c1, s1) = halfaddr(x, y);
	let (c2, s2) = halfaddr(s1, c);
	(or(c1, c2), s2)
}

pub fn add16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	let mut tmp: bool;
	let mut c = false;
	let (tmp, c) = fulladdr(x[0], y[0], c);
	ans[0] = tmp;
	let (tmp, c) = fulladdr(x[1], y[1], c);
	ans[1] = tmp;
	let (tmp, c) = fulladdr(x[2], y[2], c);
	ans[2] = tmp;
	let (tmp, c) = fulladdr(x[3], y[3], c);
	ans[3] = tmp;
	let (tmp, c) = fulladdr(x[4], y[4], c);
	ans[4] = tmp;
	let (tmp, c) = fulladdr(x[5], y[5], c);
	ans[5] = tmp;
	let (tmp, c) = fulladdr(x[6], y[6], c);
	ans[6] = tmp;
	let (tmp, c) = fulladdr(x[7], y[7], c);
	ans[7] = tmp;
	let (tmp, c) = fulladdr(x[8], y[8], c);
	ans[8] = tmp;
	let (tmp, c) = fulladdr(x[9], y[9], c);
	ans[9] = tmp;
	let (tmp, c) = fulladdr(x[10], y[10], c);
	ans[10] = tmp;
	let (tmp, c) = fulladdr(x[12], y[12], c);
	ans[11] = tmp;
	let (tmp, c) = fulladdr(x[13], y[13], c);
	ans[12] = tmp;
	let (tmp, c) = fulladdr(x[14], y[14], c);
	ans[13] = tmp;
	let (tmp, c) = fulladdr(x[14], y[14], c);
	ans[14] = tmp;
	let (tmp, c) = fulladdr(x[15], y[15], c);
	ans[15] = tmp;
	ans
}

pub fn inc16(x: [bool; 16]) -> [bool; 16]  {
	let y = [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
	add16(x, y)
}

pub fn alu(x: [bool; 16], y: [bool; 16], zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> ([bool; 16], bool, bool) {
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
	(ans, zr, ng)
}
