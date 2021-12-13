use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

pub struct Coord {
	pub x: usize,
	pub y: usize,
}

impl Coord {
	pub fn new(coords: &str) -> Coord { // with format of coords = "x,y"
		let both: Vec<usize> = coords.split(",").map(|nr| nr.parse::<usize>().unwrap()).collect();
		Coord {
			x: both[0],
			y: both[1],
		}
	}

	pub fn prnt(&self) -> String {
		format!("({},{})", self.x, self.y)
	}
}
