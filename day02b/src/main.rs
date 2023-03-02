use std::fs::read_to_string;

const FILENAME: &str = "src/input.txt";

fn main() {
    counter_play(FILENAME);
}

fn counter_play(filename: &str) -> u32 {
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

        let desired_outcome = Outcome::counter_play_outcome(&round.player_2);
        let round_points = Round::round_points(&desired_outcome);
        let move_points =
            Move::counter_move(round.player_1, desired_outcome).calculate_move_points();
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

    fn counter_move(self, outcome: Outcome) -> Move {
        match outcome {
            Outcome::Win => self.play_winning_move(),
            Outcome::Draw => self.play_draw_move(),
            Outcome::Loss => self.play_losing_move(),
        }
    }
}

impl Round {
    fn round_points(outcome: &Outcome) -> u32 {
        match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl Outcome {
    fn counter_play_outcome(player_2_move: &Move) -> Outcome {
        match player_2_move {
            Move::Rock => Outcome::Loss,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::counter_play;

    #[test]
    fn test_total_score() {
        let points = counter_play("src/test.txt");
        assert_eq!(points, 12);
    }
}
