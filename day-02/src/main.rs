use std::{fs::File, io::{BufReader, BufRead}};

enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

struct Round {
    opponent_hand: RPS,
    self_hand: RPS,
    actual_outcome: Outcome,
}

impl From<String> for Round {
    fn from(text: String) -> Self {
        match text.as_str() {
            "A X" => Round{opponent_hand: RPS::Rock, self_hand: RPS::Rock, actual_outcome: Outcome::Loss},
            "A Y" => Round{opponent_hand: RPS::Rock, self_hand: RPS::Paper, actual_outcome: Outcome::Draw},
            "A Z" => Round{opponent_hand: RPS::Rock, self_hand: RPS::Scissors, actual_outcome: Outcome::Win},
            "B X" => Round{opponent_hand: RPS::Paper, self_hand: RPS::Rock, actual_outcome: Outcome::Loss},
            "B Y" => Round{opponent_hand: RPS::Paper, self_hand: RPS::Paper, actual_outcome: Outcome::Draw},
            "B Z" => Round{opponent_hand: RPS::Paper, self_hand: RPS::Scissors, actual_outcome: Outcome::Win},
            "C X" => Round{opponent_hand: RPS::Scissors, self_hand: RPS::Rock, actual_outcome: Outcome::Loss},
            "C Y" => Round{opponent_hand: RPS::Scissors, self_hand: RPS::Paper, actual_outcome: Outcome::Draw},
            "C Z" => Round{opponent_hand: RPS::Scissors, self_hand: RPS::Scissors, actual_outcome: Outcome::Win},
            _ => panic!("Invalid round"),
        }
    }
}

impl Round {
    fn points_step1(&self) -> u32 {
        match (&self.opponent_hand, &self.self_hand) {
            (RPS::Rock, RPS::Rock) => 4, // 1 + 3
            (RPS::Rock, RPS::Paper) => 8, // 2 + 6
            (RPS::Rock, RPS::Scissors) => 3, // 3 + 0
            (RPS::Paper, RPS::Rock) => 1, // 1 + 0
            (RPS::Paper, RPS::Paper) => 5, // 2 + 3
            (RPS::Paper, RPS::Scissors) => 9, // 3 + 6
            (RPS::Scissors, RPS::Rock) => 7, // 1 + 6
            (RPS::Scissors, RPS::Paper) => 2, // 2 + 0
            (RPS::Scissors, RPS::Scissors) => 6, // 3 + 3
        }
    }

    fn points_step2(&self) -> u32 {
        match (&self.opponent_hand, &self.actual_outcome) {
            (RPS::Rock, Outcome::Loss) => 3, // 3 + 0 
            (RPS::Rock, Outcome::Draw) => 4, // 1 + 3
            (RPS::Rock, Outcome::Win) => 8, // 2 + 6
            (RPS::Paper, Outcome::Loss) => 1, // 1 + 0
            (RPS::Paper, Outcome::Draw) => 5, // 2 + 3
            (RPS::Paper, Outcome::Win) => 9, // 3 + 6
            (RPS::Scissors, Outcome::Loss) => 2, // 2 + 0
            (RPS::Scissors, Outcome::Draw) => 6, // 3 + 3
            (RPS::Scissors, Outcome::Win) => 7, // 1 + 6
        }
    }
}

fn main() {
    let file_name = "input/day-02";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let rounds = reader.lines().map(|line| Round::from(line.unwrap())).collect::<Vec<Round>>();
    let score1: u32 = rounds.iter().map(|round| round.points_step1()).sum();
    println!("Score 1: {}", score1);

    let score2: u32 = rounds.iter().map(|round| round.points_step2()).sum();
    println!("Score 2: {}", score2);
}
