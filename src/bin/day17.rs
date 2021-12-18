use std::io::{self};
use aoc::*;
use std::cmp;

fn main() -> io::Result<()> {
	// let lines:Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	let lines:Vec<String> = read_lines("inputs/input_day17.txt")?.map(Result::unwrap).collect();
	let splt: Vec<&str> = lines[0].split_whitespace().collect();
	let xstr: Vec<isize> = splt[2][2..(splt[2].len() - 1)].split("..")
		.map(|nr| nr.parse::<isize>().unwrap())
		.collect();
	let ystr: Vec<isize> = splt[3][2..(splt[3].len())].split("..")
		.map(|nr| nr.parse::<isize>().unwrap())
		.collect();
	let xrange: (isize, isize) = (xstr[0], xstr[1]);
	let yrange: (isize, isize) = (ystr[0], ystr[1]);
	let ans = day17_1(xrange, yrange);
	println!("day 17_1: {}\nday17_2: {}", ans.0, ans.1);
	Ok(())
}

fn day17_1(xrange: (isize, isize), yrange: (isize, isize)) -> (isize, isize) {
	let mut highest = 0;
	let mut total = 0;
	let mut min = 0;
	while sm(min) < xrange.0 { min += 1; }
	for x_vel in 0..(xrange.1 + 1) {
		for y_vel in yrange.0..2000 {
			let ans = try_the_ting(x_vel, y_vel, (xrange, yrange));
			if ans.0 == true {total += 1; }
			highest = cmp::max(highest, ans.1);
	 	}
	}
	(highest, total)
}

fn try_the_ting(x_vel: isize, y_vel: isize, target: ((isize, isize), (isize, isize))) -> (bool, isize) {
	let mut pos = (0, 0);
	let mut forward = x_vel;
	let mut downward = y_vel;
	let mut highest = target.1.0;
	while pos.0 <= target.0.1 && pos.1 >= target.1.0 {
		pos.0 += forward;
		pos.1 += downward;
		highest = cmp::max(highest, pos.1);
		if is_in_target(target, pos.0, pos.1) { return (true, highest); }
		if forward > 0 { forward -= 1; } else if forward < 0 { forward += 1; }
		downward -= 1;
	}
	(false, 0)
}

fn sm(n: isize) -> isize {
	(n * (n + 1)) / 2
}

fn is_in_target(target: ((isize, isize), (isize, isize)), x: isize, y: isize) -> bool {
	if x >= target.0.0 && x <= target.0.1 && y >= target.1.0 && y <= target.1.1 { true }
	else { false }
}
