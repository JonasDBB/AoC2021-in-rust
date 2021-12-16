use std::io::{self};
use aoc::*;

#[derive(Debug)]
struct Packet {
	version: u8,
	type_id: u8,
	literal: u64,
	packets: Vec<Packet>,
	len: usize,
}

impl Packet {
	fn new(bin: &Vec<char>, start: usize) -> Packet {
		let ver = u8::from_str_radix(
			&bin[start..(start + 3)].iter().collect::<String>(), 2).unwrap();
		let typ = u8::from_str_radix(
			&bin[(start + 3)..(start + 6)].iter().collect::<String>(), 2).unwrap();
		let mut lit = "".to_string();
		let mut vec= Vec::new();
		let mut index = start + 6;
		if typ == 4 {
			let mut segment_start = '1';
			while segment_start == '1' {
				segment_start = bin[index];
				for c in bin[(index + 1)..(index + 5)].iter() { lit.push(*c); }
				index += 5;
			}
		} else {
			let mode = bin[index];
			index += 1;
			if mode == '0' {
				let byte_len = usize::from_str_radix(
					&bin[index..(index + 15)].iter().collect::<String>(), 2).unwrap();
				index += 15;
				let start_section = index;
				while index - start_section < byte_len {
					let pckt = Packet::new(bin, index);
					index += &pckt.len;
					vec.push(pckt);
				}
			} else { // mode == '1'
				let packet_n = usize::from_str_radix(
					&bin[index..(index + 11)].iter().collect::<String>(), 2).unwrap();
				index += 11;
				for _ in 0..packet_n {
					let pckt = Packet::new(bin, index);
					index += &pckt.len;
					vec.push(pckt);
				}
			}
		}
		Packet {
			version: ver,
			type_id: typ,
			literal: if lit != "" { u64::from_str_radix(&lit, 2).unwrap() } else { 0 },
			packets: vec,
			len: index - start,
		}
	}

	fn version_sum(&self) -> u64 {
		&self.packets.iter()
			.map(|p| p.version_sum()).sum()
			+ self.version as u64
	}

	fn prnt(&self) -> String {
		format!("{:#?}", self)
	}
}

fn main() -> io::Result<()> {
	// let lines: Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	let lines:Vec<String> = read_lines("inputs/input_day16.txt")?.map(Result::unwrap).collect();
	let binary: Vec<char> = lines[0].chars().map(|c| hex_to_bin(c)).collect::<String>().chars().collect();
	let packet = Packet::new(&binary, 0);
	println!("day 16_1: {}", packet.version_sum());
	Ok(())
}

fn hex_to_bin(c: char) -> &'static str {
	match c {
		'0' => "0000",
		'1' => "0001",
		'2' => "0010",
		'3' => "0011",
		'4' => "0100",
		'5' => "0101",
		'6' => "0110",
		'7' => "0111",
		'8' => "1000",
		'9' => "1001",
		'A' => "1010",
		'B' => "1011",
		'C' => "1100",
		'D' => "1101",
		'E' => "1110",
		'F' => "1111",
		_ => "",
	}
}
