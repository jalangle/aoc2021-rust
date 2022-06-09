/*
jalangle@oldenberg day02 % python3 script1.py input 
Horizontal Postition: 1857
Depth: 894
H*D: 1660158
*/

#![allow(dead_code)]

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

    for i in instructions {
        if i.command == "forward" {
            horiz += i.value
        }
        else if i.command == "down" {
            depth += i.value
        }
        else if i.command == "up" {
            depth -= i.value
        }
    }
    println!("T: {}", horiz * depth);
}
