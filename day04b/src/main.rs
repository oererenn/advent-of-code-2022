use std::{fs::read_to_string};

const FILENAME: &str = "src/input.txt";

fn main() {
    let overlap_count = calculate_overlap(FILENAME);
    println!("{}", overlap_count);
}

fn calculate_overlap(filename: &str) -> i32 {
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
    .filter(|(a, b, c, d)| (d >= b && c <= b) || ( d >= a && d <= b))
    .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::calculate_overlap;

    #[test]
    fn test_sum_priorities() {
        let overlap_count = calculate_overlap("src/test.txt");
        assert_eq!(overlap_count, 4);
    }
}