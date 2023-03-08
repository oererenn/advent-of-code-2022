use std::{fs::read_to_string};

const FILENAME: &str = "src/input.txt";

fn main() {
    calculate_pairs(FILENAME);
}

fn calculate_pairs(filename: &str) -> i32 {
    read_to_string(filename).unwrap().lines().map(|l| {
        let (l, r) = l.split_once(',').unwrap();
        let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
        (
            a.parse::<u8>().unwrap(),
            b.parse::<u8>().unwrap(),
            c.parse::<u8>().unwrap(),
            d.parse::<u8>().unwrap(),
        )
    })
    .filter(|(a, b, c, d)| c >= a && d <= b || a >= c && b <= d)
    .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::calculate_pairs;

    #[test]
    fn test_sum_priorities() {
        let pair_count = calculate_pairs("src/test.txt");
        assert_eq!(pair_count, 2);
    }
}
