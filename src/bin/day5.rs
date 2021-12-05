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
	fn new(line: &String) -> Line {
		let both: Vec<&str> = line.split(" ").collect();
		Line {
			bgn: Coord::new(both[0]),
			end: Coord::new(both[2])
		}
	}

	fn prnt(&self) -> String {
		format!("{} -> {}", self.bgn.prnt(), self.end.prnt())
	}
}

fn main() -> io::Result<()> {
	let input:Vec<String> = read_lines("inputs/input_day5.txt")?.map(Result::unwrap).collect();
	day5_1(&input);
	Ok(())
}

fn day5_1(input: &Vec<String>) {
	let mut all_lines: Vec<Line> = input.iter().map(|ln| Line::new(ln)).collect();
	all_lines.retain(|line| line.bgn.x == line.end.x || line.bgn.y == line.end.y);

}