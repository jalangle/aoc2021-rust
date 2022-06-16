#[path = "util.rs"]
mod util;

struct Instance {
	pub inputs : Vec<String>,
	pub outputs : Vec<String>,
}

impl Instance {
	pub fn new(input: &String, output: &String) -> Instance {
		Instance {
			inputs: input.split(" ").map(|x| x.to_string()).collect::<Vec<String>>(),
			outputs: output.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()
		}
	}

#[allow(dead_code)]
	pub fn format(&self) -> String {
		format!("[ {} | {} ]", self.inputs.join(","), self.outputs.join(","))
	}
}

#[allow(unused_variables)]
pub fn begin(args: Vec<String>) -> u32 {

    let lines  = util::file_to_lines(&args[1]);
    let instances : Vec<Instance> = lines.iter().map(|line| {
    	let data : Vec<String> = line.split(" | ").map(|x| x.to_string()).collect::<Vec<String>>();
    	Instance::new(&data[0], &data[1])
    }).collect();
    //println!("{}", instances.iter().map(|x| x.format()).collect::<Vec<String>>().join("\n"));

    let mut digit_counts : Vec<u32> = vec![0; 10];

	for row in instances {
		for digit in row.outputs {
			if digit.len() == 2 {
				digit_counts[1] += 1
			}
			else if digit.len() == 4 {
				digit_counts[4] += 1
			}
			else if digit.len() == 3 {
				digit_counts[7] += 1
			}
			else if digit.len() == 7 {
				digit_counts[8] += 1
			}
		}
	}

	let sum = digit_counts.iter().sum();
	println!("Count: {}", sum);
	sum
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_testdata() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d8/test"];
        assert_eq!(begin(args), 26);
    }

    #[test]
    fn test_input() {
        let args : Vec<String> = vec!["modulename".to_string(), util::get_testdata_root() + "/d8/input"];
        assert_eq!(begin(args), 456);
    }
}

