use std::io::{self};
use aoc::*;

struct Map {
	w: usize,
	h: usize,
	arr: Vec<Vec<u8>>,
}

impl Map {
	fn new(lines: &Vec<String>) -> Map {
		Map {
			w: lines[0].len(),
			h: lines.len(),
			arr: lines.iter()
				.map(|row| row.split("")
					.filter(|s| !s.is_empty())
					.map(|c| c.parse::<u8>().unwrap())
					.collect())
				.collect(),
		}
	}
}

fn main() -> io::Result<()> {
	let lines:Vec<String> = read_lines("inputs/input_day9.txt")?.map(Result::unwrap).collect();
	let mut map = Map::new(&lines);
	println!("{}", day9_1(&map));
	println!("{}", day9_2(&mut map));
	Ok(())
}

fn day9_2(map: &mut Map) -> i32 {
	let mut ret: Vec<i32> = vec![0, 0, 0];
	for y in 0..map.h {
		for x in 0..map.w {
			if is_low_point(map, x, y) {
				ret.push(find_basin_size(map, x as i32, y as i32, 0));
				ret.sort();
				ret.remove(0);
			}
		}
	}
	ret[0] * ret[1] * ret[2]
}

fn find_basin_size(map: &mut Map, x: i32, y: i32, count: i32) -> i32 {
	if x < 0 || x as usize >= map.w || y < 0 || y as usize >= map.h { return count; }
	if map.arr[y as usize][x as usize] == 9 { return count; }
	let mut ret = count;
	map.arr[y as usize][x as usize] = 9;
	ret += 1;
	ret = find_basin_size(map, x + 1, y, ret);
	ret = find_basin_size(map, x - 1, y, ret);
	ret = find_basin_size(map, x, y + 1, ret);
	ret = find_basin_size(map, x, y - 1, ret);
	ret
}

fn day9_1(map: &Map) -> i32 {
	let mut ret = 0;
	for y in 0..map.h {
		for x in 0..map.w {
			if is_low_point(map, x, y) { ret += 1 + map.arr[y][x] as i32 }
		}
	}
	ret
}

fn is_low_point(map: &Map, x: usize, y: usize) -> bool {
	if x > 0 { if map.arr[y][x] >= map.arr[y][x - 1] { return false; } }
	if x < map.w - 1 { if map.arr[y][x] >= map.arr[y][x + 1] { return false; } }
	if y > 0 { if map.arr[y][x] >= map.arr[y - 1][x] { return false; } }
	if y < map.h - 1 { if map.arr[y][x] >= map.arr[y + 1][x ] { return false; } }
	true
}
