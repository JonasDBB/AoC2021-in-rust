use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let lines = get_input("inputs/input_day2.txt");
	println!("{}", day2_1(&lines));
	println!("{}", day2_2(&lines));
}

fn day2_2(lines: &Vec<String>) -> i32 {
	let mut depth = 0;
	let mut forward = 0;
	let mut aim = 0;
	for line in lines {
		let vars: Vec<&str> = line.split(' ').collect();
		let amount = vars[1].parse::<i32>().unwrap();
		if vars[0] == "up" {
			aim -= amount;
		} else if vars[0] == "down" {
			aim += amount;
		} else {
			forward += amount;
			depth += aim * amount;
		}
	}
	depth * forward
}

fn day2_1(lines: &Vec<String>) -> i32 {
	let mut depth = 0;
	let mut forward = 0;
	for line in lines {
		let vars: Vec<&str> = line.split(' ').collect();
		let amount = vars[1].parse::<i32>().unwrap();
		if vars[0] == "up" {
			depth -= amount;
		} else if vars[0] == "down" {
			depth += amount;
		} else {
			forward += amount;
		}
	}
	depth * forward
}

fn get_input(input: &str) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	if let Ok(lines) = read_lines(input) {
		for line in lines {
			if let Ok(ln) = line {
				ret.push(ln);
			}
		}
	}
	ret
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
