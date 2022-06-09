#[path = "util.rs"]
mod util;

fn add_vectors(left: Vec<u32>, right: Vec<u32>) -> Vec<u32> {

    let mut sum : Vec<u32> = vec![0;left.len()];
    for n in 0..left.len() {
        sum[n] = left[n] + right[n];
    }
    sum
}

fn get_gamma(count_of_lines: usize, count_of_ones: &Vec<u32>) -> u32
{
    let half : u32 = count_of_lines as u32 / 2;
    let mut gamma : u32 = 0;
    for i in 0..count_of_ones.len() {
        let most_common_bit : u32 = if count_of_ones[i] > half { 1 } else { 0 };
        gamma = gamma << 1;
        gamma += most_common_bit;
    } 
    gamma
}

fn get_epsilon(count_of_lines: usize, count_of_ones: &Vec<u32>) -> u32
{
    let half : u32 = count_of_lines as u32 / 2;
    let mut epsilon : u32 = 0;
    for i in 0..count_of_ones.len() {
        let least_common_bit : u32 = if count_of_ones[i] < half { 1 } else { 0 };
        epsilon = epsilon << 1;
        epsilon += least_common_bit;
    } 
    epsilon
}

pub fn begin(args: Vec<String>) {
    let lines  = util::file_to_lines(&args[1]);

    let initial : Vec<u32> = vec![0; lines[0].len()];
    let count_of_ones : Vec<u32> = lines.iter().map(|line| {
        line.chars().map(|char| { char.to_digit(10).unwrap() }).collect::<Vec<u32>>()
    }).fold(initial, |accumulator, value| add_vectors(accumulator, value));

    let gamma = get_gamma(lines.len(), &count_of_ones);
    println!("G: {}", gamma);

    let epsilon = get_epsilon(lines.len(), &count_of_ones);
    println!("G: {}", epsilon);

    println!("Power Consumption: {}", gamma * epsilon);
}
