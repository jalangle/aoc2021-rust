#[path = "util.rs"]
mod util;

pub fn begin(args: Vec<String>) {
    let lines  = util::file_to_lines(&args[1]);

    let mut prev : i32 = -1;
    let mut increase : i32 = 0;

    let lines : Vec<i32> = lines.iter().map(|x| x.parse().unwrap() ).collect();
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
