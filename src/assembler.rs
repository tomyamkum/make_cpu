use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::collections::HashMap;
use crate::util::*;

const SP: [bool; 15] = [false; 15];
const LCL: [bool; 15] = [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
const ARG: [bool; 15] = [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false];
const THIS: [bool; 15] = [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false];
const THAT: [bool; 15] = [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false];
const R0: [bool; 15] = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
const R1: [bool; 15] = [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
const R2: [bool; 15] = [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false];
const R3: [bool; 15] = [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false];
const R4: [bool; 15] = [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false];
const R5: [bool; 15] = [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false];
const R6: [bool; 15] = [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false];
const R7: [bool; 15] = [true, true, true, false, false, false, false, false, false, false, false, false, false, false, false];
const R8: [bool; 15] = [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false];
const R9: [bool; 15] = [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false];
const R10: [bool; 15] = [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false];
const R11: [bool; 15] = [true, true, false, true, false, false, false, false, false, false, false, false, false, false, false];
const R12: [bool; 15] = [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false];
const R13: [bool; 15] = [true, false, true, true, false, false, false, false, false, false, false, false, false, false, false];
const R14: [bool; 15] = [false, true, true, true, false, false, false, false, false, false, false, false, false, false, false];
const R15: [bool; 15] = [true, true, true, true, false, false, false, false, false, false, false, false, false, false, false];

pub fn assemble() -> Result<(), Box<std::error::Error>> {
	let mut pc: u16 = 0;
	let mut label: u16 = 16;
	let mut simbol = HashMap::new();
	let mut add_simbol = false;
	for result in BufReader::new(File::open("test.asm")?).lines() {
		let mut l = result?;
		l.retain(|c| c != '\t');
		l.retain(|c| c != ' ');
		let v: Vec<&str> = l.split("//").collect();
		let mut command = v[0].to_string();
		if command == "" {
			continue;
		}
		let ch1 = command.chars().next().unwrap();
		if ch1 == '(' {
			command.retain(|c| c != '(');
			command.retain(|c| c != ')');
			simbol.insert(command, pc);
		} else {
			pc += 1;
			continue;
		}
	}
	let mut f = BufWriter::new(File::create("test.hack").unwrap());
	for result in BufReader::new(File::open("test.asm")?).lines() {
		let mut l = result?;
		l.retain(|c| c != '\t');
		l.retain(|c| c != ' ');
		let v: Vec<&str> = l.split("//").collect();
		let mut command = v[0].to_string();
		println!("{:?}", &command);
		if command == "" {
			continue;
		}
		let ch1 = command.chars().next().unwrap();
		if ch1 == '(' {
			continue;
		}
		if ch1 == '@' {
			command.retain(|c| c != '@');
			match command.parse::<u16>() {
				Ok(n) => {
					f.write(format!("{:0>16b}", n).as_bytes()).unwrap();
					f.write(b"\n").unwrap();
				},
				Err(n) => {
					if !simbol.contains_key(&command) {
						simbol.insert(command, label);
						f.write(format!("{:0>16b}", label).as_bytes()).unwrap();
						f.write(b"\n").unwrap();
						label = label + 1
					} else {
						println!("{}", simbol[&command]);
						f.write(format!("{:0>16b}", simbol[&command]).as_bytes()).unwrap();
						f.write(b"\n").unwrap();
					}
							
				},
			};
		} else {
			if command.contains("=") == true {
				let v: Vec<&str> = command.split("=").collect();
				let dest = v[0];
				let comp = v[1];
				let dest_bytes = match dest {
					"null" => "000",
					"M" => "001",
					"D" => "010",
					"MD" => "011",
					"A" => "100",
					"AM" => "101",
					"AD" => "110",
					"AMD" => "111",
					_ => "",
				};
				let comp_bytes = match comp {
					"0" => "0101010",
					"1" => "0111111",
					"-1" => "0111010",
					"D" => "0001100",
					"A" => "0110000",
					"!D" => "0001101",
					"!A" => "0110001",
					"-D" => "0001111",
					"-A" => "0110011",
					"D+1" => "0011111",
					"A+1" => "0110111",
					"D-1" => "0001110",
					"A-1" => "0110010",
					"D+A" => "0000010",
					"D-A" => "0010011",
					"A-D" => "0000111",
					"D&A" => "0000000",
					"D|A" => "0010101",
					"M" => "1110000",
					"!M" => "1110001",
					"-M" => "1110011",
					"M+1" => "1110111",
					"M-1" => "1110010",
					"D+M" => "1000010",
					"D-M" => "1010011",
					"M-D" => "1000111",
					"D&M" => "1000000",
					"D|M" => "1010101",
					_ => "",
				};
				let jump_bytes = "000";
				f.write(format!("{}{}{}{}", "111", comp_bytes, dest_bytes, jump_bytes).as_bytes()).unwrap();
				f.write(b"\n").unwrap();
			}
			else {
				let v: Vec<&str> = command.split(";").collect();
				let comp = v[0];
				let jump = v[1];
				let comp_bytes = match comp {
					"0" => "0101010",
					"1" => "0111111",
					"-1" => "0111010",
					"D" => "0001100",
					"A" => "0110000",
					"!D" => "0001101",
					"!A" => "0110001",
					"-D" => "0001111",
					"-A" => "0110011",
					"D+1" => "0011111",
					"A+1" => "0110111",
					"D-1" => "0001110",
					"A-1" => "0110010",
					"D+A" => "0000010",
					"D-A" => "0010011",
					"A-D" => "0000111",
					"D&A" => "0000000",
					"D|A" => "0010101",
					"M" => "1110000",
					"!M" => "1110001",
					"-M" => "1110011",
					"M+1" => "1110111",
					"M-1" => "1110010",
					"D+M" => "1000010",
					"D-M" => "1010011",
					"M-D" => "1000111",
					"D&M" => "1000000",
					"D|M" => "1010101",
					_ => "",
				};
				let dest_bytes = "000";
				let jump_bytes = match jump {
					"null" => "000",
					"JGT" => "001",
					"JEQ" => "010",
					"JGE" => "011",
					"JLT" => "100",
					"JNE" => "101",
					"JLE" => "110", 
					"JMP" => "111",
					_ => "",
				};
				f.write(format!("{}{}{}{}", "111", comp_bytes, dest_bytes, jump_bytes).as_bytes()).unwrap();
				f.write(b"\n").unwrap();
			}
		}
	}
	let a:u16 = 3;
	println!("{:?}", u2b15(a));
	Ok(())
}
