use std::io::{self};
use aoc::*;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
	let lines:Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	let numbers: VecDeque<i32> = lines[0].split(",").map(|nr| nr.parse::<i32>().unwrap()).collect();
	day6_1(numbers);
	Ok(())
}

fn day6_1(fish: VecDeque<i32>) {

}
