use std::io::{self};
use aoc::*;

fn main() -> io::Result<()> {
	let lines:Vec<String> = read_lines("inputs/input_day6.txt")?.map(Result::unwrap).collect();
	let numbers: Vec<usize> = lines[0].split(",").map(|nr| nr.parse::<usize>().unwrap()).collect();
	println!("day 6_1: {}", day6(&numbers, 80));
	println!("day 6_2: {}", day6(&numbers, 256));
	Ok(())
}

fn day6(fish: &Vec<usize>, days: u32) -> u64 {
	let mut counters: [u64; 9] = get_start_vals(fish);
	for _ in 1..=days {
		let breeders = counters[0];
		for i in 0..8 { counters[i] = counters[i + 1] }
		counters[8] = breeders;
		counters[6] += breeders;
	}
	counters.iter().sum()
}

fn get_start_vals(input: &Vec<usize>) -> [u64; 9] {
	let mut ret: [u64; 9] = [0; 9];
	for i in 0..9 {
		ret[i] = input.iter()
			.filter(|&&nr| nr == i)
			.count() as u64
	}
	ret
}
