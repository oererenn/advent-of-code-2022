use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

fn main() {
    sum_priorities(FILENAME);
}

fn sum_priorities(filename: &str) -> u32 {
    let input = read_to_string(filename).unwrap();
    let mut result = 0;

    for i in input.lines() {
        let rucksack = Rucksack {
            first_compartment: i[..i.len() / 2].to_string().chars().collect(),
            second_compartment: i[i.len() / 2..].to_string().chars().collect(),
        };

        let common_priority = Rucksack::find_common(rucksack);
        result += Rucksack::value(common_priority).unwrap();
    }
    println!("{:?}", result);
    result
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}

impl Rucksack {
    fn find_common(self) -> char {
        *self
            .first_compartment
            .iter()
            .find(|c| self.second_compartment.contains(c))
            .unwrap()
    }

    fn value(c: char) -> Option<u32> {
        match c {
            'a'..='z' => Some((c as u8 - b'a' + 1) as u32),
            'A'..='Z' => Some((c as u8 - b'A' + 27) as u32),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sum_priorities;

    #[test]
    fn test_sum_priorities() {
        let sum = sum_priorities("src/test.txt");
        assert_eq!(sum, 157);
    }
}
