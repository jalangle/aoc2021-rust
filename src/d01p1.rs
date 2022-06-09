#[path = "util.rs"]
mod util;

pub fn begin(args: Vec<String>) {
    let lines  = util::file_to_lines(&args[1]);

    let mut prev : i32 = -1;
    let mut increase : i32 = 0;

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
