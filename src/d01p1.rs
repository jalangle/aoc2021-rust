use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn begin() {
    let args: Vec<String> = env::args().collect();
    
    let path = Path::new(&args[1]);
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => {}
    }

    let mut prev : i32 = -1;
    let mut increase : i32 = 0;

    let lines = s.lines();
    for l in lines {
        let i: i32 = l.parse().unwrap();
        if i > prev {
            if prev != -1 {
                increase += 1
            }
        }
        prev = i;
    }

    println!("{}", increase);
}
