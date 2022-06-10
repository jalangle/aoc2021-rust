#[path = "util.rs"]
mod util;

pub fn begin(args: Vec<String>) -> i32 {
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
    increase
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_testdata() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d1/test"];
        assert_eq!(begin(args), 7);
    }

    #[test]
    fn test_input() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d1/input"];
        assert_eq!(begin(args), 1665);
    }
}