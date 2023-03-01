use std::fs::read_to_string;
const FILENAME: &str = "src/input.txt";
fn main() {
    calculate_total_points(FILENAME);
    counter_play(FILENAME);
}

fn calculate_total_points(filename: &str) -> usize {
    let mut result = 0;
    for round in read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.trim())
    {
        let round = round
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let round = Round {
            player_1: Move::compare(round.chars().nth(0).unwrap()).unwrap(),
            player_2: Move::compare(round.chars().nth(1).unwrap()).unwrap(),
        };
        let move_selection_point = round.player_2.calculate_move_points();
        let outcome = round.player_1.outcome(round.player_2);

        let outcome_point = outcome.points();
        let final_point = move_selection_point + outcome_point;
        result += final_point;
    }
    println!("{:?}", result);
    result
}

fn counter_play(filename: &str) -> usize {
    let mut result = 0;
    for round in read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.trim())
    {
        let round = round
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let round = Round {
            player_1: Move::compare(round.chars().nth(0).unwrap()).unwrap(),
            player_2: Move::compare(round.chars().nth(1).unwrap()).unwrap(),
        };
        let counter_outcome = round.player_2.calculate_round();

        let counter_move = match counter_outcome {
            Outcome::Win => round.player_1.play_winning_move(),
            Outcome::Draw => round.player_1.play_draw_move(),
            Outcome::Loss => round.player_1.play_losing_move(),
        };

        let move_points = counter_move.calculate_move_points();
        let round_points = counter_outcome.points();
        result += move_points + round_points;
    }
    println!("{:?}", result);
    result
}

#[derive(Debug)]
struct Round {
    player_1: Move,
    player_2: Move,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// How many points do we get for picking that move?
    fn calculate_move_points(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn compare(c: char) -> Result<Move, (&'static str, &'static str)> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(("Invalid move", "Move must be A, B, C, X, Y, or Z")),
        }
    }

    fn calculate_round(self) -> Outcome {
        match self {
            Move::Rock => Outcome::Loss,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }

    fn play_winning_move(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn play_losing_move(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn play_draw_move(self) -> Move {
        match self {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        }
    }

    fn outcome(self, theirs: Move) -> Outcome {
        match (theirs, self) {
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

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_total_points, counter_play};

    #[test]
    fn test_total_points() {
        let points = calculate_total_points("src/test.txt");
        assert_eq!(points, 15);
    }

    #[test]
    fn test_counter_play() {
        let points = counter_play("src/test.txt");
        assert_eq!(points, 12);
    }
}
