use std::io::{self};
mod read_lines;
use read_lines::read_lines;

fn main() -> io::Result<()> {
	let mut lines:Vec<String> = read_lines("inputs/input_day4.txt")?.map(Result::unwrap).collect();
	let numbers: Vec<i32> = lines[0].split(",").map(|nr| nr.parse::<i32>().unwrap()).collect();
	lines.retain(|line| line.len() == 14);
	let boards: Vec<Vec<String>> = lines.chunks(5).map(|s| s.into()).collect();
	let mut allboards: Vec<Vec<Vec<i32>>> = boards.iter()
		.map(|board| board.iter()
			.map(|str| str.split(" ")
				.map(|str| str.trim())
				.filter(|str| !str.is_empty())
				.map(|nrstr| nrstr.parse::<i32>().unwrap())
				.collect())
			.collect())
		.collect();
	println!("{:?}", allboards[0]);
	day4_1(&lines, numbers);
	Ok(())
}

fn day4_1(lines: &Vec<String>, numbers: Vec<i32>) {
	// for i in 0..10 {
	// 	println!("len: {}, line: {}", lines[i].len(), lines[i]);
	// }

}