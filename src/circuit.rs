use crate::gate::*;

pub fn mux(x: bool, y: bool, sel: bool) -> bool {
	or(and(x, not(sel)), or(and(x, y), and(y, sel)))
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

pub fn add16(x: &Vec<bool>, y: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	let mut c = false;
	let (tmp1, tmp2) = fulladdr(x[0], y[0], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[1], y[1], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[2], y[2], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[3], y[3], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[4], y[4], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[5], y[5], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[6], y[6], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[7], y[7], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[8], y[8], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[9], y[9], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[10], y[10], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[11], y[11], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[12], y[12], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[13], y[13], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[14], y[14], c);
	c = tmp1;
	ans.push(tmp2);
	let (tmp1, tmp2) = fulladdr(x[15], y[15], c);
	c = tmp1;
	ans.push(tmp2);
	ans
}

pub fn inc16(x: &Vec<bool>) -> Vec<bool>  {
	let mut y = Vec::new();
	y.push(true);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	y.push(false);
	add16(&x, &y)
}

pub fn alu(x: &mut Vec<bool>, y: &mut Vec<bool>, zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> (Vec<bool>, bool, bool) {
	let x = and16_1(&x, not(zx));

	let x = xor16_1(&x, zx);

	let y = and16_1(&y, not(zy));

	let y = xor16_1(&y, zy);
	(x.to_vec(), true, true)
}
