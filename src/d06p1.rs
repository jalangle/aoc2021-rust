#![allow(dead_code)]

#[path = "util.rs"]
mod util;

const REPRODUCTION_RATE : i32 = 7;
const REPRODUCTION_MATURITY_RATE : i32 = 2;

struct Generation {
    count: i64,
    interval: i32,
}

impl Generation {
    pub fn new(count: i64, interval: i32) -> Generation {
        Generation {
            count: count,
            interval: interval,
        }
    }

    pub fn format(&self) -> String {
        let v = format!("{}", self.interval);
        let elements : Vec<String> = vec![v;self.count as usize];
        elements.join(",")
    }
}

fn printfish(fish: &Vec<Generation>) {
    let values = fish.iter().map(|x| x.format()).collect::<Vec<String>>().join(",");
    println!("{}", values)
}

pub fn begin(args: Vec<String>) -> i64 {

    let lines  = util::file_to_lines(&args[1]);
    let days : u32 = args[2].parse().unwrap();

    let mut lantern_fish : Vec<Generation> = lines[0].split(",").map(|x| { x.parse().unwrap()}).map(|x| { Generation::new(1,x)} ).collect::<Vec<Generation>>();

    for _d in 0..days {
//        printfish(&lantern_fish);
        let mut number_to_add : i64 = 0;

        for i in 0..lantern_fish.len() {
            lantern_fish[i].interval -= 1;
            if lantern_fish[i].interval < 0 {
                lantern_fish[i].interval = REPRODUCTION_RATE - 1;
                number_to_add += lantern_fish[i].count as i64;
            }
        }

        if number_to_add > 0 {
            lantern_fish.insert(lantern_fish.len(), Generation::new(number_to_add, REPRODUCTION_MATURITY_RATE + REPRODUCTION_RATE - 1));
        }
    }

    let population_size = lantern_fish.iter().map(|x| x.count).sum();
    println!("Population Size: {}", population_size);
    population_size
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_testdata_p1_18() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d6/test", "18".to_string() ];
        assert_eq!(begin(args), 26);
    }

    #[test]
    fn test_testdata_p1_80() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d6/test", "80".to_string() ];
        assert_eq!(begin(args), 5934);
    }

    #[test]
    fn test_input_p1() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d6/input", "80".to_string() ];
        assert_eq!(begin(args), 391888);
    }

    #[test]
    fn test_input_p2() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d6/input", "256".to_string() ];
        assert_eq!(begin(args), 1754597645339);
    }
}
