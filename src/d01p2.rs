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

    let lines : Vec<i32> = s.split("\n").collect::<Vec<&str>>().iter().map(|&x| x.parse().unwrap() ).collect();
    for l in lines.windows(3) {
        let sum = l.iter().sum();
        if sum > prev {
            if prev != -1 {
                increase += 1
            }
        }
        prev = sum
    }

    println!("{}", increase)

}
