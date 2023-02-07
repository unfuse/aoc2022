use std::{collections::HashMap, fs};

enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn value(&self) -> i32 {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn result(&self, other: &Throw) -> Outcome {
        match (self, other) {
            (Throw::Rock, Throw::Paper) => Outcome::Lose,
            (Throw::Rock, Throw::Scissors) => Outcome::Win,
            (Throw::Paper, Throw::Rock) => Outcome::Win,
            (Throw::Paper, Throw::Scissors) => Outcome::Lose,
            (Throw::Scissors, Throw::Rock) => Outcome::Lose,
            (Throw::Scissors, Throw::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn score(&self, other: &Throw) -> i32 {
        self.result(other).value() + self.value()
    }

    fn decode(&self, outcome: Outcome) -> Throw {
        match (self, outcome) {
            (Throw::Rock, Outcome::Win) => Throw::Paper,
            (Throw::Rock, Outcome::Lose) => Throw::Scissors,
            (Throw::Paper, Outcome::Win) => Throw::Scissors,
            (Throw::Paper, Outcome::Lose) => Throw::Rock,
            (Throw::Scissors, Outcome::Win) => Throw::Rock,
            (Throw::Scissors, Outcome::Lose) => Throw::Paper,
            (Throw::Rock, Outcome::Draw) => Throw::Rock,
            (Throw::Paper, Outcome::Draw) => Throw::Paper,
            (Throw::Scissors, Outcome::Draw) => Throw::Scissors,
        }
    }

    fn from(input: &str) -> Throw {
        match input {
            "A" | "X" => Throw::Rock,
            "B" | "Y" => Throw::Paper,
            "C" | "Z" => Throw::Scissors,
            _ => panic!("cannot parse sequence {}", input),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn from(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("could not parse outcome {}", input),
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Us,
    Them,
}

struct Match {
    us: Throw,
    them: Throw,
}

impl Match {
    fn _from_str_p1(input: &str) -> Match {
        match input.split_once(' ') {
            Some((them, us)) => Match::from_throws(Throw::from(us), Throw::from(them)),
            None => panic!("Could not parse match {}", input),
        }
    }

    fn from_str_p2(input: &str) -> Match {
        match input.split_once(' ') {
            Some((them, us)) => {
                let their_throw = Throw::from(them);
                Match::from_throws(their_throw.decode(Outcome::from(us)), their_throw)
            }
            None => panic!("Could not parse match {}", input),
        }
    }

    fn from_throws(us: Throw, them: Throw) -> Match {
        Match { us, them }
    }

    fn play(self) -> (Player, i32, Player, i32) {
        (
            Player::Us,
            self.us.score(&self.them),
            Player::Them,
            self.them.score(&self.us),
        )
    }
}

fn main() {
    let input: String = fs::read_to_string("src/bin/day02.txt").expect("Could not read file");
    let mut record: HashMap<Player, i32> = HashMap::from([(Player::Us, 0), (Player::Them, 0)]);

    // swap factory for p1/p2
    for line in input.lines() {
        let m: Match = Match::from_str_p2(line);
        let (p1, p1s, p2, p2s) = m.play();
        record.entry(p1).and_modify(|s| *s += p1s);
        record.entry(p2).and_modify(|s| *s += p2s);
    }

    println!(
        "our score is {}",
        record.get(&Player::Us).expect("could not find self")
    );
}
