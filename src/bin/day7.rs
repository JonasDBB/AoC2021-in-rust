use std::io::{self};
use aoc::*;

fn main() -> io::Result<()> {
	let lines:Vec<String> = read_lines("inputs/input_day7.txt")?.map(Result::unwrap).collect();
	let mut numbers: Vec<i32> = lines[0].split(",").map(|nr| nr.parse::<i32>().unwrap()).collect();
	println!("day 7_1: {}", day7_1(&mut numbers));
	println!("day 7_2: {}", day7_2(&mut numbers));
	// println!("min: {} max: {}", numbers.iter().min().unwrap(), numbers.iter().max().unwrap());

	Ok(())
}

fn day7_1(numbers: &mut Vec<i32>) -> i32 {
	numbers.sort();
	numbers.iter()
		.map(|&nr| (nr - numbers[numbers.len() / 2]).abs())
		.sum()
}

fn day7_2(numbers: &mut Vec<i32>) -> i32 {
	let pos: i32 = (numbers.iter().sum::<i32>() as f64 / (numbers.len() as f64)).floor() as i32;
	numbers.iter()
		.map(|&nr| (nr - pos).abs() * ((nr - pos).abs() + 1) / 2)
		.sum()
}
