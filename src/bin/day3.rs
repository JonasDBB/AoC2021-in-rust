use std::io::{self};
mod read_lines;
use read_lines::read_lines;

fn main() -> io::Result<()> {
	let lines = read_lines("inputs/input_day3.txt")?.map(Result::unwrap).collect();
	println!("3_1: {}", day3_1(&lines));
	println!("3_2: {}", day3_2(&lines));
	Ok(())
}

fn day3_1(lines: &Vec<String>) -> i32 {
	let bins = get_binary_vecs(lines);
	binary_to_dec(&bins.0) * binary_to_dec(&bins.1) // gamma * epsilon
}

fn day3_2(input: &Vec<String>) -> i32 {
	let mut oxy = reduce_to_1(input, input[0].len(), true).into_bytes();
	let mut co2 = reduce_to_1(input, input[0].len(), false).into_bytes();
	for i in oxy.iter_mut() {*i -= '0' as u8}
	for i in co2.iter_mut() {*i -= '0' as u8}
	binary_to_dec(&oxy) * binary_to_dec(&co2)
}

fn reduce_to_1(input: &Vec<String>, len: usize, more_common: bool) -> String {
	let mut reduce_vec = input.clone();
	let mut control_set = get_binary_vecs(&reduce_vec);
	let mut control;
	for i in 0..len {
		if more_common == true {control = control_set.0} else {control = control_set.1}
		reduce_vec.retain(|line| line.chars().nth(i).unwrap() as u8 == control[i] + '0' as u8);
		control_set = get_binary_vecs(&reduce_vec);
		if reduce_vec.len() < 2 {break}
	}
	reduce_vec[0].clone()
}

fn get_binary_vecs(input: &Vec<String>) -> (Vec<u8>, Vec<u8>) {
	let mut ones;
	let mut zero;
	let mut most_common = Vec::new(); // gamma / O2 gen rating
	let mut least_common = Vec::new(); // epsilon / CO2 scrub rating
	for i in 0..12 {
		ones = 0;
		zero = 0;
		for line in input {
			if line.chars().nth(i).unwrap() == '1' {ones += 1} else {zero += 1}
		}
		if ones >= zero {
			most_common.push(1);
			least_common.push(0);
		} else {
			most_common.push(0);
			least_common.push(1);
		}
	}
	(most_common, least_common)
}

fn binary_to_dec(vctr: &Vec<u8>) -> i32 {
	let mut ret = 0;
	let mut multi = 1;
	for i in (0..vctr.len()).rev() {
		if vctr[i] == 1 {ret += multi}
		multi *= 2;
	}
	ret
}
