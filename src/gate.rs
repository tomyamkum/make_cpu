pub fn nand(x: bool, y: bool) -> bool {
	!(x && y)
}

pub fn nand16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	[
		nand(x[0], y[0]),
		nand(x[1], y[1]),
		nand(x[2], y[2]),
		nand(x[3], y[3]),
		nand(x[4], y[4]),
		nand(x[5], y[5]),
		nand(x[6], y[6]),
		nand(x[7], y[7]),
		nand(x[8], y[8]),
		nand(x[9], y[9]),
		nand(x[10], y[10]),
		nand(x[11], y[11]),
		nand(x[12], y[12]),
		nand(x[13], y[13]),
		nand(x[14], y[14]),
		nand(x[15], y[15]),
	]
}

pub fn nand16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	[
		nand(x[0], y),
		nand(x[1], y),
		nand(x[2], y),
		nand(x[3], y),
		nand(x[4], y),
		nand(x[5], y),
		nand(x[6], y),
		nand(x[7], y),
		nand(x[8], y),
		nand(x[9], y),
		nand(x[10], y),
		nand(x[11], y),
		nand(x[12], y),
		nand(x[13], y),
		nand(x[14], y),
		nand(x[15], y),
	]
}

pub fn not(x: bool) -> bool {
	nand(x, x)
}

pub fn not16(x: [bool; 16]) -> [bool; 16] {
	[
		not(x[0]),
		not(x[1]),
		not(x[2]),
		not(x[3]),
		not(x[4]),
		not(x[5]),
		not(x[6]),
		not(x[7]),
		not(x[8]),
		not(x[9]),
		not(x[10]),
		not(x[11]),
		not(x[12]),
		not(x[13]),
		not(x[14]),
		not(x[15]),
	]
}

pub fn and(x: bool, y: bool) -> bool {
	not(nand(x, y))
}

pub fn and16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	[
		and(x[0], y[0]),
		and(x[1], y[1]),
		and(x[2], y[2]),
		and(x[3], y[3]),
		and(x[4], y[4]),
		and(x[5], y[5]),
		and(x[6], y[6]),
		and(x[7], y[7]),
		and(x[8], y[8]),
		and(x[9], y[9]),
		and(x[10], y[10]),
		and(x[11], y[11]),
		and(x[12], y[12]),
		and(x[13], y[13]),
		and(x[14], y[14]),
		and(x[15], y[15]),
	]
}

pub fn and16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	[
		and(x[0], y),
		and(x[1], y),
		and(x[2], y),
		and(x[3], y),
		and(x[4], y),
		and(x[5], y),
		and(x[6], y),
		and(x[7], y),
		and(x[8], y),
		and(x[9], y),
		and(x[10], y),
		and(x[11], y),
		and(x[12], y),
		and(x[13], y),
		and(x[14], y),
		and(x[15], y),
	]
}

pub fn or(x: bool, y: bool) -> bool {
	nand(not(x), not(y))
}

pub fn or16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	[
		or(x[0], y[0]),
		or(x[1], y[1]),
		or(x[2], y[2]),
		or(x[3], y[3]),
		or(x[4], y[4]),
		or(x[5], y[5]),
		or(x[6], y[6]),
		or(x[7], y[7]),
		or(x[8], y[8]),
		or(x[9], y[9]),
		or(x[10], y[10]),
		or(x[11], y[11]),
		or(x[12], y[12]),
		or(x[13], y[13]),
		or(x[14], y[14]),
		or(x[15], y[15]),
	]
}

pub fn or16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	[
		or(x[0], y),
		or(x[1], y),
		or(x[2], y),
		or(x[3], y),
		or(x[4], y),
		or(x[5], y),
		or(x[6], y),
		or(x[7], y),
		or(x[8], y),
		or(x[9], y),
		or(x[10], y),
		or(x[11], y),
		or(x[12], y),
		or(x[13], y),
		or(x[14], y),
		or(x[15], y),
	]
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
	[
		nor(x[0], y[0]),
		nor(x[1], y[1]),
		nor(x[2], y[2]),
		nor(x[3], y[3]),
		nor(x[4], y[4]),
		nor(x[5], y[5]),
		nor(x[6], y[6]),
		nor(x[7], y[7]),
		nor(x[8], y[8]),
		nor(x[9], y[9]),
		nor(x[10], y[10]),
		nor(x[11], y[11]),
		nor(x[12], y[12]),
		nor(x[13], y[13]),
		nor(x[14], y[14]),
		nor(x[15], y[15]),
	]
}

pub fn nor16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	[
		nor(x[0], y),
		nor(x[1], y),
		nor(x[2], y),
		nor(x[3], y),
		nor(x[4], y),
		nor(x[5], y),
		nor(x[6], y),
		nor(x[7], y),
		nor(x[8], y),
		nor(x[9], y),
		nor(x[10], y),
		nor(x[11], y),
		nor(x[12], y),
		nor(x[13], y),
		nor(x[14], y),
		nor(x[15], y),
	]
}

pub fn xor(x: bool, y: bool) -> bool {
	and(nand(x, y), or(x, y))
}

pub fn xor16_16(x: [bool; 16], y: [bool; 16]) -> [bool; 16] {
	[
		xor(x[0], y[0]),
		xor(x[1], y[1]),
		xor(x[2], y[2]),
		xor(x[3], y[3]),
		xor(x[4], y[4]),
		xor(x[5], y[5]),
		xor(x[6], y[6]),
		xor(x[7], y[7]),
		xor(x[8], y[8]),
		xor(x[9], y[9]),
		xor(x[10], y[10]),
		xor(x[11], y[11]),
		xor(x[12], y[12]),
		xor(x[13], y[13]),
		xor(x[14], y[14]),
		xor(x[15], y[15]),
	]
}

pub fn xor16_1(x: [bool; 16], y: bool) -> [bool; 16] {
	[
		xor(x[0], y),
		xor(x[1], y),
		xor(x[2], y),
		xor(x[3], y),
		xor(x[4], y),
		xor(x[5], y),
		xor(x[6], y),
		xor(x[7], y),
		xor(x[8], y),
		xor(x[9], y),
		xor(x[10], y),
		xor(x[11], y),
		xor(x[12], y),
		xor(x[13], y),
		xor(x[14], y),
		xor(x[15], y),
	]
}
