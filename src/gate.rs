pub fn nand(x: bool, y: bool) -> bool {
	!(x && y)
}

pub fn nand16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = nand(x[0], y[0]);
	ans[1] = nand(x[1], y[1]);
	ans[2] = nand(x[2], y[2]);
	ans[3] = nand(x[3], y[3]);
	ans[4] = nand(x[4], y[4]);
	ans[5] = nand(x[5], y[5]);
	ans[6] = nand(x[6], y[6]);
	ans[7] = nand(x[7], y[7]);
	ans[8] = nand(x[8], y[8]);
	ans[9] = nand(x[9], y[9]);
	ans[10] = nand(x[10], y[10]);
	ans[11] = nand(x[11], y[11]);
	ans[12] = nand(x[12], y[12]);
	ans[13] = nand(x[13], y[13]);
	ans[14] = nand(x[14], y[14]);
	ans[15] = nand(x[15], y[15]);
	ans
}

pub fn nand16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = nand(x[0], y);
	ans[1] = nand(x[1], y);
	ans[2] = nand(x[2], y);
	ans[3] = nand(x[3], y);
	ans[4] = nand(x[4], y);
	ans[5] = nand(x[5], y);
	ans[6] = nand(x[6], y);
	ans[7] = nand(x[7], y);
	ans[8] = nand(x[8], y);
	ans[9] = nand(x[9], y);
	ans[10] = nand(x[10], y);
	ans[11] = nand(x[11], y);
	ans[12] = nand(x[12], y);
	ans[13] = nand(x[13], y);
	ans[14] = nand(x[14], y);
	ans[15] = nand(x[15], y);
	ans
}

pub fn not(x: bool) -> bool {
	nand(x, x)
}

pub fn not16(x: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = not(x[0]);
	ans[1] = not(x[1]);
	ans[2] = not(x[2]);
	ans[3] = not(x[3]);
	ans[4] = not(x[4]);
	ans[5] = not(x[5]);
	ans[6] = not(x[6]);
	ans[7] = not(x[7]);
	ans[8] = not(x[8]);
	ans[9] = not(x[9]);
	ans[10] = not(x[10]);
	ans[11] = not(x[11]);
	ans[12] = not(x[12]);
	ans[13] = not(x[13]);
	ans[14] = not(x[14]);
	ans[15] = not(x[15]);
	ans
}

pub fn and(x: bool, y: bool) -> bool {
	not(nand(x, y))
}

pub fn and16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = and(x[0], y[0]);
	ans[1] = and(x[1], y[1]);
	ans[2] = and(x[2], y[2]);
	ans[3] = and(x[3], y[3]);
	ans[4] = and(x[4], y[4]);
	ans[5] = and(x[5], y[5]);
	ans[6] = and(x[6], y[6]);
	ans[7] = and(x[7], y[7]);
	ans[8] = and(x[8], y[8]);
	ans[9] = and(x[9], y[9]);
	ans[10] = and(x[10], y[10]);
	ans[11] = and(x[11], y[11]);
	ans[12] = and(x[12], y[12]);
	ans[13] = and(x[13], y[13]);
	ans[14] = and(x[14], y[14]);
	ans[15] = and(x[15], y[15]);
	ans
}

pub fn and16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = and(x[0], y);
	ans[1] = and(x[1], y);
	ans[2] = and(x[2], y);
	ans[3] = and(x[3], y);
	ans[4] = and(x[4], y);
	ans[5] = and(x[5], y);
	ans[6] = and(x[6], y);
	ans[7] = and(x[7], y);
	ans[8] = and(x[8], y);
	ans[9] = and(x[9], y);
	ans[10] = and(x[10], y);
	ans[11] = and(x[11], y);
	ans[12] = and(x[12], y);
	ans[13] = and(x[13], y);
	ans[14] = and(x[14], y);
	ans[15] = and(x[15], y);
	ans
}

pub fn or(x: bool, y: bool) -> bool {
	nand(not(x), not(y))
}

pub fn or16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = or(x[0], y[0]);
	ans[1] = or(x[1], y[1]);
	ans[2] = or(x[2], y[2]);
	ans[3] = or(x[3], y[3]);
	ans[4] = or(x[4], y[4]);
	ans[5] = or(x[5], y[5]);
	ans[6] = or(x[6], y[6]);
	ans[7] = or(x[7], y[7]);
	ans[8] = or(x[8], y[8]);
	ans[9] = or(x[9], y[9]);
	ans[10] = or(x[10], y[10]);
	ans[11] = or(x[11], y[11]);
	ans[12] = or(x[12], y[12]);
	ans[13] = or(x[13], y[13]);
	ans[14] = or(x[14], y[14]);
	ans[15] = or(x[15], y[15]);
	ans
}

pub fn or16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = or(x[0], y);
	ans[1] = or(x[1], y);
	ans[2] = or(x[2], y);
	ans[3] = or(x[3], y);
	ans[4] = or(x[4], y);
	ans[5] = or(x[5], y);
	ans[6] = or(x[6], y);
	ans[7] = or(x[7], y);
	ans[8] = or(x[8], y);
	ans[9] = or(x[9], y);
	ans[10] = or(x[10], y);
	ans[11] = or(x[11], y);
	ans[12] = or(x[12], y);
	ans[13] = or(x[13], y);
	ans[14] = or(x[14], y);
	ans[15] = or(x[15], y);
	ans
}

pub fn or16way(x: [bool; 16]) -> bool {
	let mut ans = false;
	ans = or(ans, x[0]);
	ans = or(ans, x[1]);
	ans = or(ans, x[2]);
	ans = or(ans, x[3]);
	ans = or(ans, x[4]);
	ans = or(ans, x[5]);
	ans = or(ans, x[6]);
	ans = or(ans, x[7]);
	ans = or(ans, x[8]);
	ans = or(ans, x[9]);
	ans = or(ans, x[10]);
	ans = or(ans, x[11]);
	ans = or(ans, x[12]);
	ans = or(ans, x[13]);
	ans = or(ans, x[14]);
	ans = or(ans, x[15]);
	ans
}

pub fn or8way(x: [bool; 8]) -> bool {
	let mut ans = false;
	ans = or(x[0], ans);
	ans = or(x[1], ans);
	ans = or(x[2], ans);
	ans = or(x[3], ans);
	ans = or(x[4], ans);
	ans = or(x[5], ans);
	ans = or(x[6], ans);
	ans = or(x[7], ans);
	ans
}

pub fn nor(x: bool, y: bool) -> bool {
	not(or(x, y))
}

pub fn nor16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = nor(x[0], y[0]);
	ans[1] = nor(x[1], y[1]);
	ans[2] = nor(x[2], y[2]);
	ans[3] = nor(x[3], y[3]);
	ans[4] = nor(x[4], y[4]);
	ans[5] = nor(x[5], y[5]);
	ans[6] = nor(x[6], y[6]);
	ans[7] = nor(x[7], y[7]);
	ans[8] = nor(x[8], y[8]);
	ans[9] = nor(x[9], y[9]);
	ans[10] = nor(x[10], y[10]);
	ans[11] = nor(x[11], y[11]);
	ans[12] = nor(x[12], y[12]);
	ans[13] = nor(x[13], y[13]);
	ans[14] = nor(x[14], y[14]);
	ans[15] = nor(x[15], y[15]);
	ans
}

pub fn nor16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = nor(x[0], y);
	ans[1] = nor(x[1], y);
	ans[2] = nor(x[2], y);
	ans[3] = nor(x[3], y);
	ans[4] = nor(x[4], y);
	ans[5] = nor(x[5], y);
	ans[6] = nor(x[6], y);
	ans[7] = nor(x[7], y);
	ans[8] = nor(x[8], y);
	ans[9] = nor(x[9], y);
	ans[10] = nor(x[10], y);
	ans[11] = nor(x[11], y);
	ans[12] = nor(x[12], y);
	ans[13] = nor(x[13], y);
	ans[14] = nor(x[14], y);
	ans[15] = nor(x[15], y);
	ans
}

pub fn xor(x: bool, y: bool) -> bool {
	and(nand(x, y), or(x, y))
}

pub fn xor16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = xor(x[0], y[0]);
	ans[1] = xor(x[1], y[1]);
	ans[2] = xor(x[2], y[2]);
	ans[3] = xor(x[3], y[3]);
	ans[4] = xor(x[4], y[4]);
	ans[5] = xor(x[5], y[5]);
	ans[6] = xor(x[6], y[6]);
	ans[7] = xor(x[7], y[7]);
	ans[8] = xor(x[8], y[8]);
	ans[9] = xor(x[9], y[9]);
	ans[10] = xor(x[10], y[10]);
	ans[11] = xor(x[11], y[11]);
	ans[12] = xor(x[12], y[12]);
	ans[13] = xor(x[13], y[13]);
	ans[14] = xor(x[14], y[14]);
	ans[15] = xor(x[15], y[15]);
	ans
}

pub fn xor16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	let mut ans: [bool; 16] = [true; 16];
	ans[0] = xor(x[0], y);
	ans[1] = xor(x[1], y);
	ans[2] = xor(x[2], y);
	ans[3] = xor(x[3], y);
	ans[4] = xor(x[4], y);
	ans[5] = xor(x[5], y);
	ans[6] = xor(x[6], y);
	ans[7] = xor(x[7], y);
	ans[8] = xor(x[8], y);
	ans[9] = xor(x[9], y);
	ans[10] = xor(x[10], y);
	ans[11] = xor(x[11], y);
	ans[12] = xor(x[12], y);
	ans[13] = xor(x[13], y);
	ans[14] = xor(x[14], y);
	ans[15] = xor(x[15], y);
	ans
}
