// https://adventofcode.com/2021/day/7

#[path = "util.rs"]
mod util;

fn cost_to_move_from(from:&i32, to:i32) -> i32{
	return (from - to).abs()
}

pub fn begin(args: Vec<String>) -> i32 {
    let lines  = util::file_to_lines(&args[1]);
    let crab_positions : Vec<i32> = lines[0].split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let max_position : usize = (*crab_positions.iter().max().unwrap()) as usize;

	let mut position_costs : Vec<i32> = vec![0;max_position as usize];
	for p in 0..max_position {
		for crab in &crab_positions {
			position_costs[p] += cost_to_move_from(crab, p as i32);
		}
	}

	let minimum_cost = position_costs.iter().min().unwrap();
	println!("{}", minimum_cost);
	*minimum_cost
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_testdata() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d7/test"];
        assert_eq!(begin(args), 37);
    }

    #[test]
    fn test_input() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d7/input"];
        assert_eq!(begin(args), 325528);
    }
}
