use std::io::{self};
use aoc::*;

struct Cave {
	id: String,
	small: bool,
	connections: Vec<String>,
}

impl Cave {
	fn new(name: &str) -> Cave {
		Cave {
			id: name.to_string(),
			small: if name.len() == 2 &&
				name.chars().next().unwrap().is_ascii_lowercase() {
				true
			} else {
				false
			},
			connections: Vec::new(),
		}
	}
}

fn main() -> io::Result<()> {
	let lines: Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	// let lines: Vec<String> = read_lines("inputs/input_day12.txt")?.map(Result::unwrap).collect();
	// println!("day12_1: {}", day12_1(&lines));
	day12_1(&lines);
	Ok(())
}

fn day12_1(lines: &Vec<String>) {
	let mut cvs = make_caves(lines);
	let start = cvs.iter().find(|cv| cv.id == "start").unwrap();
	// for connec in &start.connections {
	// 	// println!("{}", connec);
	// 	let mut path =
	// }
	findpaths(start);
}

fn find_paths(cv: &Cave) -> i32 {


	0
}

fn make_caves(lines: &Vec<String>) -> Vec<Cave> {
	let mut cvs: Vec<Cave> = Vec::new();
	for line in lines {
		let both:Vec<&str> = line.split("-").collect();
		if !cvs.iter().any(|cv| cv.id == both[0]) {
			cvs.push(Cave::new(both[0]));
		}
		if !cvs.iter().any(|cv| cv.id == both[1]) {
			cvs.push(Cave::new(both[1]));
		}
		cvs.iter_mut().find(|cv| cv.id == both[0]).unwrap().connections.push(both[1].to_string());
		cvs.iter_mut().find(|cv| cv.id == both[1]).unwrap().connections.push(both[0].to_string());
	}
	cvs
}
