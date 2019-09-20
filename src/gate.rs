pub fn nand(x: bool, y: bool) -> bool {
	!(x && y)
}

pub fn nand16_16(x: &Vec<bool>, y: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(nand(x[0], y[0]));
	ans.push(nand(x[1], y[1]));
	ans.push(nand(x[2], y[2]));
	ans.push(nand(x[3], y[3]));
	ans.push(nand(x[4], y[4]));
	ans.push(nand(x[5], y[5]));
	ans.push(nand(x[6], y[6]));
	ans.push(nand(x[7], y[7]));
	ans.push(nand(x[8], y[8]));
	ans.push(nand(x[9], y[9]));
	ans.push(nand(x[10], y[10]));
	ans.push(nand(x[11], y[11]));
	ans.push(nand(x[12], y[12]));
	ans.push(nand(x[13], y[13]));
	ans.push(nand(x[14], y[14]));
	ans.push(nand(x[15], y[15]));
	ans
}

pub fn nand16_1(x: &Vec<bool>, y: bool) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(nand(x[0], y));
	ans.push(nand(x[1], y));
	ans.push(nand(x[2], y));
	ans.push(nand(x[3], y));
	ans.push(nand(x[4], y));
	ans.push(nand(x[5], y));
	ans.push(nand(x[6], y));
	ans.push(nand(x[7], y));
	ans.push(nand(x[8], y));
	ans.push(nand(x[9], y));
	ans.push(nand(x[10], y));
	ans.push(nand(x[11], y));
	ans.push(nand(x[12], y));
	ans.push(nand(x[13], y));
	ans.push(nand(x[14], y));
	ans.push(nand(x[15], y));
	ans
}

pub fn not(x: bool) -> bool {
	nand(x, x)
}

pub fn not16(x: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(not(x[0]));
	ans.push(not(x[1]));
	ans.push(not(x[2]));
	ans.push(not(x[3]));
	ans.push(not(x[4]));
	ans.push(not(x[5]));
	ans.push(not(x[6]));
	ans.push(not(x[7]));
	ans.push(not(x[8]));
	ans.push(not(x[9]));
	ans.push(not(x[10]));
	ans.push(not(x[11]));
	ans.push(not(x[12]));
	ans.push(not(x[13]));
	ans.push(not(x[14]));
	ans.push(not(x[15]));
	ans
}

pub fn and(x: bool, y: bool) -> bool {
	not(nand(x, y))
}

pub fn and16_16(x: &Vec<bool>, y: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(and(x[0], y[0]));
	ans.push(and(x[1], y[1]));
	ans.push(and(x[2], y[2]));
	ans.push(and(x[3], y[3]));
	ans.push(and(x[4], y[4]));
	ans.push(and(x[5], y[5]));
	ans.push(and(x[6], y[6]));
	ans.push(and(x[7], y[7]));
	ans.push(and(x[8], y[8]));
	ans.push(and(x[9], y[9]));
	ans.push(and(x[10], y[10]));
	ans.push(and(x[11], y[11]));
	ans.push(and(x[12], y[12]));
	ans.push(and(x[13], y[13]));
	ans.push(and(x[14], y[14]));
	ans.push(and(x[15], y[15]));
	ans
}

pub fn and16_1(x: &Vec<bool>, y: bool) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(and(x[0], y));
	ans.push(and(x[1], y));
	ans.push(and(x[2], y));
	ans.push(and(x[3], y));
	ans.push(and(x[4], y));
	ans.push(and(x[5], y));
	ans.push(and(x[6], y));
	ans.push(and(x[7], y));
	ans.push(and(x[8], y));
	ans.push(and(x[9], y));
	ans.push(and(x[10], y));
	ans.push(and(x[11], y));
	ans.push(and(x[12], y));
	ans.push(and(x[13], y));
	ans.push(and(x[14], y));
	ans.push(and(x[15], y));
	ans
}

pub fn or(x: bool, y: bool) -> bool {
	nand(not(x), not(y))
}

pub fn or16_16(x: &Vec<bool>, y: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(or(x[0], y[0]));
	ans.push(or(x[1], y[1]));
	ans.push(or(x[2], y[2]));
	ans.push(or(x[3], y[3]));
	ans.push(or(x[4], y[4]));
	ans.push(or(x[5], y[5]));
	ans.push(or(x[6], y[6]));
	ans.push(or(x[7], y[7]));
	ans.push(or(x[8], y[8]));
	ans.push(or(x[9], y[9]));
	ans.push(or(x[10], y[10]));
	ans.push(or(x[11], y[11]));
	ans.push(or(x[12], y[12]));
	ans.push(or(x[13], y[13]));
	ans.push(or(x[14], y[14]));
	ans.push(or(x[15], y[15]));
	ans
}

pub fn or16_1(x: &Vec<bool>, y: bool) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(or(x[0], y));
	ans.push(or(x[1], y));
	ans.push(or(x[2], y));
	ans.push(or(x[3], y));
	ans.push(or(x[4], y));
	ans.push(or(x[5], y));
	ans.push(or(x[6], y));
	ans.push(or(x[7], y));
	ans.push(or(x[8], y));
	ans.push(or(x[9], y));
	ans.push(or(x[10], y));
	ans.push(or(x[11], y));
	ans.push(or(x[12], y));
	ans.push(or(x[13], y));
	ans.push(or(x[14], y));
	ans.push(or(x[15], y));
	ans
}

pub fn or1_16(x: &Vec<bool>) -> bool {
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

pub fn nor(x: bool, y: bool) -> bool {
	not(or(x, y))
}

pub fn nor16_16(x: &Vec<bool>, y: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(nor(x[0], y[0]));
	ans.push(nor(x[1], y[1]));
	ans.push(nor(x[2], y[2]));
	ans.push(nor(x[3], y[3]));
	ans.push(nor(x[4], y[4]));
	ans.push(nor(x[5], y[5]));
	ans.push(nor(x[6], y[6]));
	ans.push(nor(x[7], y[7]));
	ans.push(nor(x[8], y[8]));
	ans.push(nor(x[9], y[9]));
	ans.push(nor(x[10], y[10]));
	ans.push(nor(x[11], y[11]));
	ans.push(nor(x[12], y[12]));
	ans.push(nor(x[13], y[13]));
	ans.push(nor(x[14], y[14]));
	ans.push(nor(x[15], y[15]));
	ans
}

pub fn nor16_1(x: &Vec<bool>, y: bool) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(nor(x[0], y));
	ans.push(nor(x[1], y));
	ans.push(nor(x[2], y));
	ans.push(nor(x[3], y));
	ans.push(nor(x[4], y));
	ans.push(nor(x[5], y));
	ans.push(nor(x[6], y));
	ans.push(nor(x[7], y));
	ans.push(nor(x[8], y));
	ans.push(nor(x[9], y));
	ans.push(nor(x[10], y));
	ans.push(nor(x[11], y));
	ans.push(nor(x[12], y));
	ans.push(nor(x[13], y));
	ans.push(nor(x[14], y));
	ans.push(nor(x[15], y));
	ans
}

pub fn xor(x: bool, y: bool) -> bool {
	and(nand(x, y), or(x, y))
}

pub fn xor16_16(x: &Vec<bool>, y: &Vec<bool>) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(xor(x[0], y[0]));
	ans.push(xor(x[1], y[1]));
	ans.push(xor(x[2], y[2]));
	ans.push(xor(x[3], y[3]));
	ans.push(xor(x[4], y[4]));
	ans.push(xor(x[5], y[5]));
	ans.push(xor(x[6], y[6]));
	ans.push(xor(x[7], y[7]));
	ans.push(xor(x[8], y[8]));
	ans.push(xor(x[9], y[9]));
	ans.push(xor(x[10], y[10]));
	ans.push(xor(x[11], y[11]));
	ans.push(xor(x[12], y[12]));
	ans.push(xor(x[13], y[13]));
	ans.push(xor(x[14], y[14]));
	ans.push(xor(x[15], y[15]));
	ans
}

pub fn xor16_1(x: &Vec<bool>, y: bool) -> Vec<bool> {
	let mut ans = Vec::new();
	ans.push(xor(x[0], y));
	ans.push(xor(x[1], y));
	ans.push(xor(x[2], y));
	ans.push(xor(x[3], y));
	ans.push(xor(x[4], y));
	ans.push(xor(x[5], y));
	ans.push(xor(x[6], y));
	ans.push(xor(x[7], y));
	ans.push(xor(x[8], y));
	ans.push(xor(x[9], y));
	ans.push(xor(x[10], y));
	ans.push(xor(x[11], y));
	ans.push(xor(x[12], y));
	ans.push(xor(x[13], y));
	ans.push(xor(x[14], y));
	ans.push(xor(x[15], y));
	ans
}
