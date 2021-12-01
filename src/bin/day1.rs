use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	println!("1_1: {}", day1_1("inputs/input_day1.txt"));
	println!("1_2: {}", day1_2("inputs/input_day1.txt"));
}

fn day1_2(input: &str) -> i32 {
	let nrs = get_nrs(input);
	let mut result = 0;
	let mut prev_sum: i32 = nrs[0..3].iter().sum();
	for wndw in nrs.windows(3) {
		let new_sum: i32 = wndw.iter().sum();
		if new_sum > prev_sum {
			result += 1;
		}
		prev_sum = new_sum;
	}
	result
}

fn day1_1(input: &str) -> i32 {
	let nrs = get_nrs(input);
	let mut result = 0;
	for wndw in nrs.windows(2) {
		if wndw[1] > wndw[0] {
			result += 1;
		}
	}
	result
}

fn get_nrs(input: &str) -> Vec<i32> {
	let mut nrs: Vec<i32> = Vec::new();
	if let Ok(lines) = read_lines(input) {
		for line in lines {
			if let Ok(nr) = line {
				let intnr = nr.parse::<i32>().unwrap();
				nrs.push(intnr);
			}
		}
	}
	nrs
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
