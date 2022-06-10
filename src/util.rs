#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn file_to_lines(path: &String) -> Vec<String> {
    let path = Path::new(path);

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

pub fn get_testdata_root() -> String {
    let path_env = env::var("AOC_TESTDATA");

    if path_env.is_ok() {
    }  else {
        println!("Env var not found");
    }

    path_env.unwrap()
}