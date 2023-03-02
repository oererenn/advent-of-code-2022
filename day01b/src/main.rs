use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";

fn main() {
    top_tree_sum(FILENAME);
}

fn top_tree_sum(filename: &str) -> u64 {
    let lines = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut elf_lead = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();

    elf_lead.sort_unstable();
    let result = elf_lead.into_iter().rev().take(3).sum();
    println!("{:?}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::top_tree_sum;

    #[test]
    fn test_top_three_sum() {
        let result = top_tree_sum("src/test.txt");
        assert_eq!(result, 45000)
    }
}
