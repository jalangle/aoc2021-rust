#[path = "util.rs"]
mod util;

pub fn begin(args: Vec<String>) -> i32 {
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

    println!("{}", increase);
    return increase
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_testdata() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d1/test"];
        assert_eq!(begin(args), 5);
    }

    #[test]
    fn test_input() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d1/input"];
        assert_eq!(begin(args), 1702);
    }
}