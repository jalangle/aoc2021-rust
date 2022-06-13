#![allow(dead_code)]

use std::collections::HashMap;

#[path = "util.rs"]
mod util;

struct Point {
    pub x : i32,
    pub y : i32
}

impl Point {
    #[allow(dead_code)]
    pub fn new(_coords: &String) -> Point {
        let parts = _coords.split(",").collect::<Vec<&str>>();

        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn format(&self) -> String  {
        let result = format!("({}, {})", self.x, self.y);
        result
    }
}

fn is_horizontal(start: &Point, end: &Point) -> bool {
    start.x == end.x
}

fn is_vertical(start: &Point, end: &Point) -> bool {
    start.y == end.y
}


#[allow(unused_must_use)]
pub fn begin(args: Vec<String>) -> u32 {
    let lines  = util::file_to_lines(&args[1]);

    let parts = lines.iter().map(|x| {
        let parts = x.split(" -> ").map(|x| x.to_string() ).collect::<Vec<String>>();
        let start = Point::new(&parts[0]);
        let end = Point::new(&parts[1]);
        (start, end)
    });

    let mut visited_points = HashMap::new();

    for p in parts {
//        println!("{} => {}", p.0.format(), p.1.format());
        if is_horizontal(&p.0, &p.1) {
            let y_direction : i32 = if p.0.y > p.1.y { -1 } else { 1 };
            let mut y = p.0.y;
            loop {
                let key = format!("({}, {})", p.0.x, y);
//                println!("K: {}", key);
                let point = visited_points.entry(key).or_insert(0);
                *point += 1;
                y += y_direction;
                if y == p.1.y + y_direction {
                    break;
                }
            }
        }
        else if is_vertical(&p.0, &p.1) {
            let x_direction : i32 = if p.0.x > p.1.x { -1 } else { 1 };
            let mut x = p.0.x;

            loop {
                let key = format!("({}, {})", x, p.0.y);
//                println!("K: {}", key);
                let point = visited_points.entry(key).or_insert(0);
                *point += 1;
                x += x_direction;
                if x == p.1.x + x_direction {
                    break;
                }
            }
        }
        else {
            let x_direction : i32 = if p.0.x > p.1.x { -1 } else { 1 };
            let y_direction : i32 = if p.0.y > p.1.y { -1 } else { 1 };

            let mut x = p.0.x;
            let mut y = p.0.y;

            loop {
                let key = format!("({}, {})", x, y);
//                println!("K: {}", key);
                let point = visited_points.entry(key).or_insert(0);
                *point += 1;
                x += x_direction;
                y += y_direction;
                if x == p.1.x + x_direction {
                    break;
                }
            }

//            println!("SKIP: {} => {}", p.0.format(), p.1.format());
        }
    }

    let overlapping = visited_points.values().filter(|x| { x > &&1 } ).count() as u32;
    println!("Overlapping: {}", overlapping);

    overlapping
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_testdata() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d5/test"];
        assert_eq!(begin(args), 12);
    }

    #[test]
    fn test_input() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d5/input"];
        assert_eq!(begin(args), 22116);
    }
}
