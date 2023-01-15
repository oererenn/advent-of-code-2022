use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "./test.txt";
    let mut cal_vec = return_max_cal_vec(filename);
    first_part(&cal_vec);
    second_part(&mut cal_vec);
}

// Calculate the maximum calories
fn first_part(calories: &Vec<i32>) -> &i32{
    let max = calories.iter().max().expect("No max found");
    max
}

// Calculate the top 3 calories
fn second_part(calories: &mut Vec<i32>) -> i32{
    let sorted_vec = calories;
    sorted_vec.sort_by(|a, b| b.cmp(a));
    let sum = sorted_vec.iter().take(3).fold(0, |acc, x| acc + x);
    sum
}

// Return a vector of the maximum calories
fn return_max_cal_vec(filename: &str) -> Vec<i32> {
    let mut cal = 0;
    let mut cal_vec: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 0 {
                    cal = cal + ip.parse::<i32>().expect("Failed to parse the string to i32");
                } else {
                    cal_vec.push(cal);
                    cal = 0;
                }
            }
        }
        if cal > 0 {
            cal_vec.push(cal);
        }
    }

    cal_vec
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{first_part, return_max_cal_vec, second_part};

    #[test]
    fn find_max_cal() {
        let vec = return_max_cal_vec("./test.txt");
        let max = first_part(&vec);
        let result = max.clone();
        assert_eq!(result, 24000);
    }

    #[test]
    fn top_three_sum() {
        let mut vec = return_max_cal_vec("./test.txt");
        let max = second_part(&mut vec);
        assert_eq!(max, 45000)
    }
}