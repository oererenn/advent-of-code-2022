use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    find_max_cal(FILENAME);
}

fn find_max_cal(filename: &str) -> u64 {
    let lines = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let elf_lead = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max()
        .unwrap();
    println!("{elf_lead}");
    elf_lead
}

#[cfg(test)]
mod tests {
    use crate::find_max_cal;

    #[test]
    fn test_find_max_cal() {
        let result = find_max_cal("src/test.txt");
        assert_eq!(result, 24000);
    }
}
