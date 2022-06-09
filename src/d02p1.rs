/*
jalangle@oldenberg day02 % python3 script1.py input 
Horizontal Postition: 1857
Depth: 894
H*D: 1660158
*/

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

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

fn file_to_lines(path: &Path) -> Vec<String> {

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => {}
    };

    let lines : Vec<String> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
    lines
}

pub fn begin(args: Vec<String>) {
    
    let path = Path::new(&args[1]);
    let lines = file_to_lines(path);

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
