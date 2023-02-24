fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    first_part(&lines);
    second_part(&lines);
}

fn first_part(lines: &Vec<Option<u64>>) -> u64 {
    let elf_lead = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max()
        .unwrap();
    println!("{}", elf_lead);
    elf_lead
}

fn second_part(lines: &Vec<Option<u64>>) -> u64 {
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
    use crate::{first_part, second_part};

    fn input() ->Vec<Option<u64>> {
        let lines = include_str!("../test.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();
        lines
    }

    #[test]
    fn find_max_cal() {
        let input = input();
        let max = first_part(&input);
        assert_eq!(max, 24000);
    }

    #[test]
    fn top_three_sum() {
        let input = input();
        let max = second_part(&input);
        assert_eq!(max, 45000)
    }
}
