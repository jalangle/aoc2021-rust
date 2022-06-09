/*
jalangle@oldenberg day02 % python3 script2.py
Horizontal Postition: 1857
Depth: 864078
H*D: 1604592846
*/

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::path::Path;
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

pub fn begin(args: Vec<String>) {
    
    let path = Path::new(&args[1]);
    let lines = util::file_to_lines(path);

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
}
