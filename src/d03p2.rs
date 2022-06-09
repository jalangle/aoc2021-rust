#[path = "util.rs"]
mod util;

fn add_vectors(left: Vec<u32>, right: &Vec<u32>) -> Vec<u32> {

    let mut sum : Vec<u32> = vec![0;left.len()];
    for n in 0..left.len() {
        sum[n] = left[n] + right[n];
    }
    sum
}

fn binary_to_number(value: &Vec<u32>) -> u32 {
    let mut result : u32 = 0;
    for n in 0..value.len() {
        result = result << 1;
        result += value[n];
    }

    if value.len() == 5 {
        println!("[ {} {} {} {} {} ] => {}", value[0], value[1], value[2], value[3], value[4], result);
    }
    else if value.len() == 7 {
        println!("[ {} {} {} {} {} {} {} ] => {}", value[0], value[1], value[2], value[3], value[4], value[5], value[6], result);        
    }

    return result;
}

fn get_o2rating(values: Vec<Vec<u32>>) -> u32 {
    let mut filtered_values = values.clone();

    for n in 0..filtered_values[0].len() {
        let initial : Vec<u32> = vec![0; values[0].len()];
        let count_of_ones : u32 = filtered_values.iter().fold(initial, |accumulator, value| add_vectors(accumulator, value))[n];
        let count_of_zeros : u32 = filtered_values.len() as u32 - count_of_ones;
        let most_common_value : u32 = if count_of_zeros > count_of_ones { 0 } else { 1 };
        filtered_values.retain(|x| x[n] == most_common_value);
        if filtered_values.len() == 1 {
            break;
        }
    }

    binary_to_number(&filtered_values[0])
}

fn get_co2rating(values: Vec<Vec<u32>>) -> u32 {
    let mut filtered_values = values.clone();

    for n in 0..filtered_values[0].len() {
        let initial : Vec<u32> = vec![0; values[0].len()];
        let count_of_ones : u32 = filtered_values.iter().fold(initial, |accumulator, value| add_vectors(accumulator, value))[n];
        let count_of_zeros : u32 = filtered_values.len() as u32 - count_of_ones;
        let most_common_value : u32 = if count_of_zeros > count_of_ones { 1 } else { 0 };
        filtered_values.retain(|x| x[n] == most_common_value);
        if filtered_values.len() == 1 {
            break;
        }
    }

    binary_to_number(&filtered_values[0])
}


pub fn begin(args: Vec<String>) {
    
    let lines  = util::file_to_lines(&args[1]);

    let values : Vec<Vec<u32>> = lines.iter().map(|line| {
        line.chars().map(|char| { char.to_digit(10).unwrap() }).collect::<Vec<u32>>()
    }).collect();

    let o2rating = get_o2rating(values.clone());
    println!("O2: {}", o2rating);

    let co2rating = get_co2rating(values.clone());
    println!("CO2: {}", co2rating);

    println!("Life Support Rating: {}", o2rating * co2rating);
}
