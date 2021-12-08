extern crate itertools;

use std::io::{self};
use aoc::*;
use itertools::Itertools;

struct Entry {
	patterns: Vec<String>,
	digits: Vec<String>,
}

impl Entry {
	fn new(line: &String) -> Entry {
		let all: Vec<&str> = line.split("|").collect();
		Entry {
			patterns: all[0].split_whitespace()
				.map(|ln| ln.chars()
					.sorted()
					.collect::<String>())
				.collect(),
			digits: all[1].split_whitespace()
				.map(|ln| ln.chars()
					.sorted()
					.collect::<String>())
				.collect(),
		}
	}
}

fn main() -> io::Result<()> {
	let lines:Vec<String> = read_lines("inputs/input_day8.txt")?.map(Result::unwrap).collect();
	println!("day 8_1: {}", day8_1(&lines));
	println!("day 8_2: {}", day8_2(&lines));
	Ok(())
}

fn day8_2(lines: &Vec<String>) -> i32{
	lines.iter()
		.map(|ln| Entry::new(ln))
		.collect::<Vec<Entry>>()// vec of all entries
		.iter()
		.map(|e| find_code(e))// find all codes as ints
		.sum()
}

fn find_code(entry: &Entry) -> i32 {
	let mut strings: [String; 10] = Default::default();
	// setup known digits
	strings[1] = entry.patterns.iter().filter(|p| p.len() == 2).nth(0).unwrap().to_string();
	strings[4] = entry.patterns.iter().filter(|p| p.len() == 4).nth(0).unwrap().to_string();
	strings[7] = entry.patterns.iter().filter(|p| p.len() == 3).nth(0).unwrap().to_string();
	strings[8] = entry.patterns.iter().filter(|p| p.len() == 7).nth(0).unwrap().to_string();
	for pattern in &entry.patterns {
		// skip known digits
		if pattern.len() == 2 || pattern.len() == 3 || pattern.len() == 4 || pattern.len() == 7 {continue}
		if pattern.len() == 5 { // 2, 3 or 5
			if shared_chars(&strings[1], &pattern) == 2 {
				strings[3] = pattern.to_string(); // 1 and 3 match 2chars, both 2 and 5 match 1char with 1
			} else if shared_chars(&strings[4], &pattern) == 2 {
				strings[2] = pattern.to_string(); // 2 and 4 match 2chars, 5 and 4 match 3chars
			} else {
				strings[5] = pattern.to_string(); // other of len 5
			}
		} else { // len is 6, so 0, 6 or 9
			if shared_chars(&strings[1], &pattern) == 1 {
				strings[6] = pattern.to_string(); // 1 and 6 match 1, both 0 and 9 match 2chars with 1
			} else if shared_chars(&strings[4], &pattern) == 3 {
				strings[0] = pattern.to_string(); // 0 and 4 match 3chars, 9 and 4 match 4chars
			} else {
				strings[9] = pattern.to_string(); // other len of 6
			}
		}
	}
	// create code as string per char, convert to int at once on return
	let mut ret_s = "".to_string();
	for digit in &entry.digits {
		for i in 0..10 {
			if strings[i] == *digit {ret_s.push((i as u8 + '0' as u8) as char)}
		}
	}
	ret_s.parse::<i32>().unwrap()
}

fn shared_chars(digit: &String, pattern: &String) -> i8 {
	let mut ret = 0;
	for char in digit.chars() {
		if pattern.contains(char) {ret += 1}
	}
	ret
}

fn day8_1(lines: &Vec<String>) -> i32 {
	let (mut ones, mut fours, mut sevens, mut eights) = (0, 0, 0, 0);
	let entries: Vec<Entry> = lines.iter().map(|ln| Entry::new(ln)).collect();
	for entry in entries {
		ones += entry.digits.iter()
			.filter(|s| s.len() == 2)
			.count();
		fours += entry.digits.iter()
			.filter(|s| s.len() == 4)
			.count();
		sevens += entry.digits.iter()
			.filter(|s| s.len() == 3)
			.count();
		eights += entry.digits.iter()
			.filter(|s| s.len() == 7)
			.count();
	}
	(ones + fours + sevens + eights) as i32
}
