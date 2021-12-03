use std::io::{self};
mod read_lines;
use read_lines::read_lines;

fn main() -> io::Result<()> {
	let lines = read_lines("inputs/input_day2.txt")?.map(Result::unwrap).collect();
	println!("2_1: {}", day2_1(&lines));
	println!("2_2: {}", day2_2(&lines));
	Ok(())
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
