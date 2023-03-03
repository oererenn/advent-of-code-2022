use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

fn main() {
    sum_priorities(FILENAME);
}

fn sum_priorities(filename: &str) -> u32 {
    // Elfs divided into 3 groups
    // Every Elf carrying item type Badge
    // At most two elfs carring other item type
    // Find the common between all three elfs in each group
    // Every tree line means a single group
    //

    let input = read_to_string(filename).unwrap();
    let input_vec = input.lines().collect::<Vec<&str>>();
    let input_chunk = input_vec.chunks(3);
    let mut result = 0;

    for i in input_chunk {
        let mut group = Group {
            badge: ' ',
            first_elf: Rucksack {
                items: i[0][..].to_string().chars().collect(),
            },
            second_elf: Rucksack {
                items: i[1][..].to_string().chars().collect(),
            },
            third_elf: Rucksack {
                items: i[2][..].to_string().chars().collect(),
            },
        };

        group.badge = Group::find(&group);
        result += Rucksack::value(group.badge).unwrap();
    }
    println!("{:#?}", result);
    result
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<char>,
}

struct Group {
    badge: char,
    first_elf: Rucksack,
    second_elf: Rucksack,
    third_elf: Rucksack,
}

impl Group {
    fn find(&self) -> char {
        *self
            .first_elf
            .items
            .iter()
            .find(|c| self.second_elf.items.contains(c) && self.third_elf.items.contains(c))
            .unwrap()
    }
}

impl Rucksack {
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
        assert_eq!(sum, 70);
    }
}
