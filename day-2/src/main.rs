use std::error::Error;

fn main() {
    for round in include_str!("input.txt").lines().map(|line| line.trim()) {
        let round =  round.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let round = Round {
            player_1: Move::compare(round.chars().nth(0).unwrap()).unwrap(),
            player_2: Move::compare(round.chars().nth(1).unwrap()).unwrap(),
        };

        
        println!("{:?}",round);
    }
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
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    
    fn compare(c: char) -> Result<Move, (&'static str, &'static str)>{
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(("Invalid move", "Move must be A, B, C, X, Y, or Z")),
        }
    }

    fn outcome(self, theirs: Move) -> Outcome {
        match (self, theirs) {
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
