use std::io::{self};
use aoc::*;

fn main() -> io::Result<()> {
	let mut lines:Vec<String> = read_lines("inputs/input_day4.txt")?.map(Result::unwrap).collect();
	let numbers: Vec<i32> = lines[0].split(",").map(|nr| nr.parse::<i32>().unwrap()).collect();
	lines.retain(|line| line.len() == 14);
	println!("day 4_1: {}", day4_1(&lines, &numbers));
	println!("day 4_2: {}", day4_2(&lines, &numbers));
	Ok(())
}

fn day4_1(lines: &Vec<String>, numbers: &Vec<i32>) -> i32 {
	let mut boards = get_boards(lines);
	for n in numbers {
		for i in 0..boards.len() {
			boards[i] = cross_number_on_board(&boards[i], &n);
			if check_if_bingo(&boards[i]) {
				// println!("BINGO on board {}\n with sum {}", i, leftover_nr_sum(&boards[i]));
				return n * leftover_nr_sum(&boards[i]);
			}
		}
	}
	0
}

fn day4_2(lines: &Vec<String>, numbers: &Vec<i32>) -> i32 {
	let mut boards = get_boards(lines);
	let mut already_found = Vec::new();
	for n in numbers {
		for i in 0..boards.len() {
			if already_found.iter().find(|&&boardnr| boardnr == i) != None {continue}
			boards[i] = cross_number_on_board(&boards[i], &n);
			if check_if_bingo(&boards[i]) {
				if already_found.len() == 99 {
					// println!("nr was {} on board nr {} with board\n{:?\n}", n, i, &boards[i]);
					return n * leftover_nr_sum(&boards[i]);
				}
				already_found.push(i);
			}
		}
	}
	0
}

fn cross_number_on_board(board: &Vec<Vec<i32>>, nr: &i32) -> Vec<Vec<i32>>{ //crossing will set it to -1
	board.iter()
		.map(|row| row.iter()
			.map(|&nb| if nb == *nr {-1} else {nb})
			.collect()
		).collect()
}

fn check_if_bingo(board: &Vec<Vec<i32>>) -> bool {
	for i in 0..5 {
		if board[i][0] == -1 && board[i][1] == -1 && board[i][2] == -1 && board[i][3] == -1 && board[i][4] == -1 {
			return true;
		}
		if board[0][i] == -1 && board[1][i] == -1 && board[2][i] == -1 && board[3][i] == -1 && board[4][i] == -1 {
			return true;
		}
	}
	false
}

fn leftover_nr_sum(board: &Vec<Vec<i32>>) -> i32 { // sum all nrs that are >= 0
	board.iter()
		.map(|row| row.iter()
			.filter(|&&nr| nr >= 0)
			.sum())
		.collect::<Vec<i32>>()
		.iter()
		.sum()
}

fn get_boards(lines: &Vec<String>) -> Vec<Vec<Vec<i32>>> { // create vec of boards where a board is a vec of vec of i32
	let str_boards: Vec<Vec<String>> = lines.chunks(5).map(|s| s.into()).collect();
	str_boards.iter()
		.map(|board| board.iter()
			.map(|str| str.split(" ")
				.map(|str| str.trim())
				.filter(|str| !str.is_empty())
				.map(|nrstr| nrstr.parse::<i32>().unwrap())
				.collect())
			.collect())
		.collect()
}
