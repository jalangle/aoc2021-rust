/*
jalangle@oldenberg day02 % python3 script2.py
Horizontal Postition: 1857
Depth: 864078
H*D: 1604592846
*/

#![allow(dead_code)]

#[path = "util.rs"]
mod util;

struct Instruction {
    pub command: String,
    pub value: i32,
}

impl Instruction {
    pub fn new(line: String) -> Instruction {
        let parts : Vec<&str> = line.split(' ').collect::<Vec<&str>>();
        
        Instruction {
            command: parts[0].to_string(),
            value: parts[1].parse().unwrap(),
        }
    }

    pub fn format(&self) -> String  {
        format!("{} : {}", self.command, self.value)
    }
}

pub fn begin(args: Vec<String>) -> i32 {
    let lines = util::file_to_lines(&args[1]);

    let instructions : Vec<Instruction> = lines.iter().map(|x| Instruction::new(x.to_string()) ).collect();

    let mut horiz : i32 = 0;
    let mut depth : i32 = 0;
    let mut aim : i32 = 0;

    for i in instructions {
        if i.command == "forward" {
            horiz += i.value;
            depth += aim * i.value;
        }
        else if i.command == "down" {
            aim += i.value;
        }
        else if i.command == "up" {
            aim -= i.value;
        }
    }
    println!("T: {}", horiz * depth);
    return horiz * depth
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_testdata() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d2/test"];
        assert_eq!(begin(args), 900);
    }

    #[test]
    fn test_input() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d2/input"];
        assert_eq!(begin(args), 1604592846);
    }
}
