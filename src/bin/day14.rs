use std::io::{self};
use aoc::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let mut lines:Vec<String> = read_lines("inputs/input_day14.txt")?.map(Result::unwrap).collect();
	let start_poly = lines[0].to_string();
	lines.retain(|ln| ln.len() == 7);
	let mut ele_map :HashMap<&str, &str> = HashMap::new();
	for line in &lines {
		let mut splt = line.split(" ");
		ele_map.insert(splt.next().unwrap(), splt.last().unwrap());
	}
	println!("day 14_1: {}", doshit(&start_poly, &ele_map, 10));
	println!("day 14_2: {}", doshit(&start_poly, &ele_map, 40));
	Ok(())
}

fn doshit(input: &String, ele_map: &HashMap<&str, &str>, steps: usize) -> usize{
	let mut counters: HashMap<&str, usize> = HashMap::new();
	for pair in ele_map {
		counters.insert(pair.0, 0);
	}
	for wndw in input.as_bytes().windows(2) {
		let key = std::str::from_utf8(wndw).unwrap();
		*counters.get_mut(key).unwrap() += 1;
	}
	let mut new_counters = HashMap::new();
	for _ in 0..steps {
		new_counters = counters.clone();
		for pair in ele_map {
			let key = *pair.0;
			let newpair1 = format!("{}{}", &pair.0[0..1], *pair.1);
			let newpair2 = format!("{}{}", *pair.1, &pair.0[1..2]);
			let n = counters[key];
			*new_counters.get_mut(key).unwrap() -= n;
			*new_counters.get_mut(&newpair1[..]).unwrap() += n;
			*new_counters.get_mut(&newpair2[..]).unwrap() += n;
		}
		counters = new_counters;
	}
	get_answer(input, &counters)
}

fn get_answer(input: &String, counters: &HashMap<&str, usize>) -> usize {
	let mut totals: HashMap<char, usize> = HashMap::new();
	for pair in counters {
		for c in pair.0.chars() { totals.insert(c, 0); }
	}
	for pair in counters {
		for c in pair.0.chars() {
			*totals.get_mut(&c).unwrap() += pair.1;
		}
	}
	*totals.get_mut(&input.chars().next().unwrap()).unwrap() += 1;
	*totals.get_mut(&input.chars().last().unwrap()).unwrap() += 1;
	let mut max = 0;
	let mut min = totals[&input.chars().next().unwrap()];
	for mut pair in totals {
		pair.1 /= 2;
		max = if pair.1 > max { pair.1 } else { max };
		min = if pair.1 < min { pair.1 } else { min };
	}
	max - min
}
