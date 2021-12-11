use std::io::{self};
use aoc::*;

struct Squid {
	n: u8,
	total: u32,
}

impl Squid {
	fn new(nr: u8) -> Squid {
		Squid {
			n: nr,
			total: 0,
		}
	}

	fn prnt(&self) -> String { format!("{}", self.n) }
}

fn main() -> io::Result<()> {
	let lines: Vec<String> = read_lines("inputs/input_day11.txt")?.map(Result::unwrap).collect();
	println!("day11_1: {}", day11_1(&lines));
	println!("day11_2: {}", day11_2(&lines));
	Ok(())
}

fn day11_2(lines: & Vec<String>) -> u32 {
	let mut squids: Vec<Vec<Squid>> = lines.iter()
		.map(|ln| ln.split("")
			.filter(|c| !c.is_empty())
			.map(|c| Squid::new(c.parse::<u8>().unwrap()))
			.collect())
		.collect();
	let mut ret = 0;
	while have_al_flashed(&squids) == false {
		for y in 0..10 {
			for x in 0..10 { flash_sqd(&mut squids, x, y); }
		}
		reset_field(&mut squids);
		ret += 1;
	}
	ret
}

fn have_al_flashed(squids: &Vec<Vec<Squid>>) -> bool {
	for y in 0..10 {
		for x in 0..10 {
			if squids[y][x].n != 0 { return false; }
		}
	}
	true
}

fn day11_1(lines: & Vec<String>) -> u32 {
	let mut squids: Vec<Vec<Squid>> = lines.iter()
		.map(|ln| ln.split("")
			.filter(|c| !c.is_empty())
			.map(|c| Squid::new(c.parse::<u8>().unwrap()))
			.collect())
		.collect();
	for _ in 0..100 { // steps
		for y in 0..10 {
			for x in 0..10 { flash_sqd(&mut squids, x, y); }
		}
		reset_field(&mut squids);
	}
	let mut ret = 0;
	for y in 0..10 {
		for x in 0..10 { ret += squids[y][x].total; }
	}
	ret
}

fn flash_sqd(squids: &mut Vec<Vec<Squid>>, got_x: i8, got_y: i8) {
	if got_x < 0 || got_y < 0 || got_x > 9 || got_y > 9 { return; }
	let x = got_x as usize;
	let y = got_y as usize;
	if squids[y][x].n > 10 { panic!("should never happen"); }
	if squids[y][x].n == 10 { return; }
	squids[y][x].n += 1;
	if squids[y][x].n == 10 {
		squids[y][x].total += 1;
		flash_sqd(squids, got_x + 1, got_y);
		flash_sqd(squids, got_x - 1, got_y);
		flash_sqd(squids, got_x, got_y + 1);
		flash_sqd(squids, got_x, got_y - 1);
		flash_sqd(squids, got_x + 1, got_y + 1);
		flash_sqd(squids, got_x + 1, got_y - 1);
		flash_sqd(squids, got_x - 1, got_y + 1);
		flash_sqd(squids, got_x - 1, got_y - 1);
	}
}

fn reset_field(squids: &mut Vec<Vec<Squid>>) {
	for y in 0..10 {
		for x in 0..10 {
			if squids[y][x].n == 10 { squids[y][x].n = 0; } }
	}
}
