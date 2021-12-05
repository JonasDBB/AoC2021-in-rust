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
			y: both[1]
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
			end: Coord::new(line.split(" ").nth(2).unwrap())
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
	let input:Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	println!("{}", day5_1(&input));
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

}

fn draw_line(line: &Line, field: &mut Field) {
	if line.bgn.x == line.end.x {
		if line.bgn.y <= line.end.y {
			for i in line.bgn.y..=line.end.y {
				field.f[i as usize][line.bgn.x as usize] += 1;
			}
		} else {
			for i in line.end.y..=line.bgn.y {
				field.f[i as usize][line.bgn.x as usize] += 1;
			}
		}
	} else if line.bgn.y == line.end.y {
		if line.bgn.x <= line.end.x {
			for i in line.bgn.x..=line.end.x {
				field.f[line.bgn.y as usize][i as usize] += 1;
			}
		} else {
			for i in line.end.x..=line.bgn.x {
				field.f[line.bgn.y as usize][i as usize] += 1;
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