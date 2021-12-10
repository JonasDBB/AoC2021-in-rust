use std::io::{self};
use aoc::*;

fn main() -> io::Result<()> {
	let mut lines:Vec<String> = read_lines("inputs/input_day10.txt")?.map(Result::unwrap).collect();
	println!("{}", day10_1(&lines));
	println!("{}", day10_2(&mut lines));
	Ok(())
}

fn day10_2(lines: &mut Vec<String>) -> u64 {
	lines.retain(|line| find_corrupted(&line) == 0);
	let mut scores: Vec<u64> = lines.iter()
		.map(|line| get_score(line))
		.collect();
	scores.sort();
	scores[scores.len() / 2]
}

fn get_score(line: &String) -> u64 {
	let mut opening: Vec<char> = Vec::new();
	for c in line.chars() {
		match c {
			'(' => { opening.push(c); },
			'[' => { opening.push(c); },
			'{' => { opening.push(c); },
			'<' => { opening.push(c); },
			')' => { opening.pop(); },
			']' => { opening.pop(); },
			'}' => { opening.pop(); },
			'>' => { opening.pop(); },
			_ => panic!("found unknown char {}", c),
		}
	}
	let mut ret = 0;
	for c in opening.iter().rev() {
		ret *= 5;
		match c {
			'(' => ret += 1,
			'[' => ret += 2,
			'{' => ret += 3,
			'<' => ret += 4,
			_ => panic!("found unknown char {}", c),
		}
	}
	ret
}

fn day10_1(lines: &Vec<String>) -> i32 {
	lines.iter()
		.map(|line| find_corrupted(line))
		.sum()
}

fn find_corrupted(line: &String) -> i32 {
	let mut opening: Vec<char> = Vec::new();
	for c in line.chars() {
		match c {
			'(' => opening.push(c),
			'[' => opening.push(c),
			'{' => opening.push(c),
			'<' => opening.push(c),
			')' => if opening[opening.len() - 1] == '(' { opening.pop(); } else { return 3 },
			']' => if opening[opening.len() - 1] == '[' { opening.pop(); } else { return 57 },
			'}' => if opening[opening.len() - 1] == '{' { opening.pop(); } else { return 1197 },
			'>' => if opening[opening.len() - 1] == '<' { opening.pop(); } else { return 25137 },
			_ => panic!("found unknown char {}", c),
		}
	}
	0
}
