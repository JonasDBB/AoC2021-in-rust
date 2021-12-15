use std::io::{self};
use aoc::*;
use std::cmp;
use std::time::Instant;

fn main() -> io::Result<()> {
	// let lines:Vec<String> = read_lines("inputs/example.txt")?.map(Result::unwrap).collect();
	let lines:Vec<String> = read_lines("inputs/input_day15.txt")?.map(Result::unwrap).collect();
	let cave: Vec<Vec<u32>> = lines.iter()
		.map(|ln| ln.split("")
			.filter(|s| !s.is_empty())
			.map(|c| c.parse::<u32>().unwrap())
			.collect())
		.collect();
	let now1 = Instant::now();
	println!("day 15_1: {}", day15_1(&cave));
	println!("runtime part1: {}", now1.elapsed().as_secs_f32());
	let now2 = Instant::now();
	println!("day 15_2: {}", day15_2(&cave));
	println!("runtime: {}", now2.elapsed().as_secs_f32());
	Ok(())
}

fn day15_2(tile: &Vec<Vec<u32>>) -> u32{
	let mut endcave = tile.to_vec();
	for n in 1..5 {
		for y in 0..tile.len() {
			for x in 0..tile[0].len() {
				let mut val = tile[y][x] + n;
				if val > 9 { val -= 9 }
				endcave[y].push(val);
			}
		}
	}
	for n in 1..5 {
		for i in 0..tile.len() {
			endcave.push(
				endcave[i].to_vec().into_iter()
					.map(|x| if x + n > 9 { x + n - 9 } else { x + n })
					.collect()
			)
		}
	}
	day15_1(&endcave)
}

fn stuff(current: &(usize, usize), neighbor: &(usize, usize),
		 val_map: &mut Vec<Vec<u32>>,
		 cave: &Vec<Vec<u32>>) -> bool {
	if val_map[neighbor.1][neighbor.0] > val_map[current.1][current.0] + cave[neighbor.1][neighbor.0] {
		val_map[neighbor.1][neighbor.0] = cmp::min(val_map[neighbor.1][neighbor.0],
										   val_map[current.1][current.0] + cave[neighbor.1][neighbor.0]);
		true
	} else { false }
}

fn day15_1(cave: &Vec<Vec<u32>>) -> u32 {
	println!("doing {} by {}", cave[0].len(), cave.len());
	let mut val_map: Vec<Vec<u32>> = vec![vec![u32::MAX; cave[0].len()]; cave.len()];
	val_map[0][0] = 0;
	let w = cave[0].len() - 1;
	let h = cave.len() - 1;
	let mut open: Vec<(usize, usize)> = Vec::new();
	let mut current = (0, 0); // (x, y)
	while current != (w, h) {
		if current.0 > 0 {
			let left = (current.0 - 1, current.1);
			if stuff(&current, &left, &mut val_map, cave) { open.push(left); }
		}
		if current.0 < w {
			let right = (current.0 + 1, current.1);
			if stuff(&current, &right, &mut val_map, cave) { open.push(right); }
		}
		if current.1 > 0 {
			let up = (current.0, current.1 - 1);
			if stuff(&current, &up, &mut val_map, cave) { open.push(up); }
		}
		if current.1 < h {
			let down = (current.0, current.1 + 1);
			if stuff(&current, &down, &mut val_map, cave) { open.push(down); }
		}
		let mut dist = u32::MAX;
		for loc in &open {
			let newdist = val_map[loc.1][loc.0];
			if newdist < dist {
				dist = newdist;
				current = *loc;
			}
		}
		open.retain(|&loc| loc != current);
	}
	val_map[val_map.len() - 1][val_map[0].len() - 1]
}
