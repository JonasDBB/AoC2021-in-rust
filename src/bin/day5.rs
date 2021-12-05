use std::io::{self};
use aoc::*;

struct Coord {
	x: i32,
	y: i32,
}

impl Coord {
	fn new(coords: &str) -> Coord { // with format of coords = "x,y"
		let both: Vec<i32> = coords.split(",").map(|nr| nr.parse::<i32>().unwrap()).collect();
		Coord {
			x: both[0],
			y: both[1],
		}
	}

	fn copy(&self) -> Coord {
		Coord {
			x: self.x,
			y: self.y,
		}
	}

	fn prnt(&self) -> String {
		format!("({},{})", self.x, self.y)
	}
}

struct Line {
	bgn: Coord,
	end: Coord,
}

impl Line {
	fn new(line: &String) -> Line { // with format of line = "x,y -> x,y"
		Line {
			bgn: Coord::new(line.split(" ").nth(0).unwrap()),
			end: Coord::new(line.split(" ").nth(2).unwrap()),
		}
	}

	fn prnt(&self) -> String {
		format!("{} -> {}", self.bgn.prnt(), self.end.prnt())
	}
}

const FIELD_SIZE: usize = 1000;
struct Field {
	f: Vec<Vec<i32>> // using an array will overflow stack
}

impl Field {
	fn new() -> Field {
		Field {
			f: vec![vec![0; FIELD_SIZE]; FIELD_SIZE]
		}
	}
}

fn main() -> io::Result<()> {
	let input:Vec<String> = read_lines("inputs/input_day5.txt")?.map(Result::unwrap).collect();
	println!("day 5_1: {}", day5_1(&input));
	println!("day 5_2: {}", day5_2(&input));
	Ok(())
}

fn day5_1(input: &Vec<String>) -> i32 {
	let mut all_lines: Vec<Line> = input.iter().map(|ln| Line::new(ln)).collect();
	let mut field = Field::new();
	all_lines.retain(|line| line.bgn.x == line.end.x || line.bgn.y == line.end.y);
	for line in all_lines {
		draw_line(&line, &mut field);
	}
	count_higher_than_1(&field)
}

fn day5_2(input: &Vec<String>) -> i32 {
	let all_lines: Vec<Line> = input.iter().map(|ln| Line::new(ln)).collect();
	let mut field = Field::new();
	for line in all_lines {
		draw_line(&line, &mut field);
	}

	count_higher_than_1(&field)
}

fn draw_line(line: &Line, field: &mut Field) {
	let mut bgn = line.bgn.copy();
	let mut end = line.end.copy();
	if bgn.x == end.x {
		if bgn.y > end.y {std::mem::swap(&mut bgn.y, &mut end.y)}
		for i in bgn.y..=end.y {
			field.f[i as usize][bgn.x as usize] += 1;
		}
	} else if bgn.y == end.y {
		if bgn.x > end.x {std::mem::swap(&mut bgn.x, &mut end.x)}
		for i in bgn.x..=end.x {
			field.f[bgn.y as usize][i as usize] += 1;
		}
	} else { // diagonal line only
		let x_forward = if bgn.x <= end.x {true} else {false};
		let mut j = bgn.x;
		if bgn.y > end.y {
			for i in (end.y..=bgn.y).rev() {
				field.f[i as usize][j as usize] += 1;
				if x_forward == true {j += 1} else {j -= 1}
			}
		} else {
			for i in bgn.y..=end.y {
				field.f[i as usize][j as usize] += 1;
				if x_forward == true {j += 1} else {j -= 1}
			}
		}
	}
}

fn count_higher_than_1(field: &Field) -> i32 {
	field.f.iter()
		.map(|row| row.iter()
			.filter(|&&nr| nr >= 2)
			.count() as i32)
		.collect::<Vec<i32>>()
		.iter()
		.sum()
}