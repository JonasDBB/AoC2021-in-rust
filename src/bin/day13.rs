use std::io::{self};
use aoc::*;
use aoc::Coord;

fn main() -> io::Result<()> {
	// let lines: Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	let lines:Vec<String> = read_lines("inputs/input_day13.txt")?.map(Result::unwrap).collect();
	let index = lines.iter().position(|ln| ln.is_empty()).unwrap();
	let coords: Vec<Coord> = lines[0..index].iter()
		.map(|ln| Coord::new(ln)).collect();
	let mut folds: Vec<(char, usize)> = Vec::new();
	for fld in lines[index + 1..lines.len()]
		.iter().map(|ln| ln
		.split(" ")
		.nth(2)
		.unwrap()
		.split_at(2))
		.collect::<Vec<(&str, &str)>>() {
		let mut nw: (char, usize) = ('0', 0);
		nw.0 = fld.0.chars().next().unwrap();
		nw.1 = fld.1.parse::<usize>().unwrap();
		folds.push(nw);
	}
	let mut max_x = 0;
	let mut max_y = 0;
	for fld in &folds {
		if fld.0 == 'x' && max_x == 0 { max_x = fld.1 * 2 + 1; }
		if fld.0 == 'y' && max_y == 0 { max_y = fld.1 * 2 + 1; }
	}
	println!("{} {}", max_x, max_y);
	let mut paper: Vec<Vec<char>> = vec![vec!['.'; max_x]; max_y];
	for crd in coords { paper[crd.y][crd.x] = '#'; }
	println!("day 13_1: {}", day13_1(&mut paper, &folds[0]));
	day13_2(&mut paper, &folds[1..folds.len()].to_vec());
	Ok(())
}

fn day13_2(paper: &mut Vec<Vec<char>>, folds: &Vec<(char, usize)>) {
	for fld in folds {
		if fld.0 == 'y'{ fold_horizontal(paper, fld.1); }
		else { fold_vertical(paper, fld.1); }
	}
	prnfld(paper);
}

fn day13_1(paper: &mut Vec<Vec<char>>, first_fold: &(char, usize)) -> usize {
	if first_fold.0 == 'y' { fold_horizontal(paper, first_fold.1); }
	else { fold_vertical(paper, first_fold.1); }
	nr_of_dots(&paper)
}

fn fold_horizontal(paper: &mut Vec<Vec<char>>, fold: usize) {
	let mut n = paper.len() - 1;
	for i in 0..fold {
		for x in 0..paper[0].len() {
			if paper[n][x] == '#' { paper[i][x] = '#'; }
		}
		n -= 1;
	}
	paper.drain(fold..paper.len());
}

fn fold_vertical(paper: &mut Vec<Vec<char>>, fold: usize) {
	for y in 0..paper.len() {
		let mut n = paper[0].len() - 1;
		for x in 0..fold {
			if paper[y][n] == '#' { paper[y][x] = '#'; }
			n -= 1;
		}
	}
	for row in paper {
		row.drain(fold..row.len());
	}
}

fn nr_of_dots(f: &Vec<Vec<char>>) -> usize {
	f.iter()
		.map(|row| row.iter()
			.filter(|&&c| c == '#')
			.count())
		.collect::<Vec<usize>>()
		.iter()
		.sum()
}

fn prnfld(f: &Vec<Vec<char>>) {
	for y in 0..f.len() {
		for x in 0..f[0].len() {
			print!("{} ", f[y][x]);
		}
		print!("\n");
	}
}
