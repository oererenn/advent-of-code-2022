use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

fn main() {
    total_score(FILENAME);
}

fn total_score(filename: &str) -> u32 {
    let mut total_score = 0;
    let input = read_to_string(filename).unwrap();

    for i in input.lines() {
        let round_string = i
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();

        let round = Round {
            player_1: Move::find_move(round_string[0]),
            player_2: Move::find_move(round_string[1]),
        };

        let outcome = Round::round_outcome(&round.player_1, &round.player_2);
        let round_points = Round::round_points(outcome);
        let move_points = Move::calculate_move_points(&round.player_2);
        total_score += round_points + move_points;
    }
    println!("{total_score}");
    total_score
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
struct Round {
    player_1: Move,
    player_2: Move,
}

impl Move {
    fn calculate_move_points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn find_move(c: char) -> Move {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Invalid Move"),
        }
    }
}

impl Round {
    fn round_points(outcome: Outcome) -> u32 {
        match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn round_outcome(player_1: &Move, player_2: &Move) -> Outcome {
        match (player_2, player_1) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::total_score;

    #[test]
    fn test_total_score() {
        let points = total_score("src/test.txt");
        assert_eq!(points, 15);
    }
}
