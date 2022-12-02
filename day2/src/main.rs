use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
enum RPS {
    Rock, Paper, Scissors
}

impl RPS {
    fn of(c: char) -> RPS {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            _ => panic!("Ahhh")
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win, Lose, Draw
}

impl Outcome {
    fn score(&self) -> i32 {
        match &self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3
        }
    }

    fn of(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Ooops")
        }
    }
}

fn resolve(p1: &RPS, p2: &RPS) -> Outcome {
    match p1 {
        RPS::Rock => {
            match p2 {
                RPS::Rock => Outcome::Draw,
                RPS::Paper => Outcome::Lose,
                RPS::Scissors=> Outcome::Win
            }
        },
        RPS::Paper => {
            match p2 {
                RPS::Rock => Outcome::Win,
                RPS::Paper => Outcome::Draw,
                RPS::Scissors=> Outcome::Lose
            }
        },
        RPS::Scissors => {
            match p2 {
                RPS::Rock => Outcome::Lose,
                RPS::Paper => Outcome::Win,
                RPS::Scissors=> Outcome::Draw
            }
        }
    }
}

fn resolve_my_play(outcome: &Outcome, opponent: &RPS) -> RPS {
    match opponent {
        RPS::Rock => {
            match outcome {
                Outcome::Win => RPS::Paper,
                Outcome::Lose => RPS::Scissors,
                Outcome::Draw => RPS::Rock
            }
        },
        RPS::Paper => {
            match outcome {
                Outcome::Win => RPS::Scissors,
                Outcome::Lose => RPS::Rock,
                Outcome::Draw => RPS::Paper
            }
        },
        RPS::Scissors => {
            match outcome {
                Outcome::Win => RPS::Rock,
                Outcome::Lose => RPS::Paper,
                Outcome::Draw => RPS::Scissors
            }
        }
    }
}

fn reader(path: &str) -> BufReader<File> {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn part1() {
    println!("Hello, world!");
    let mut total_score = 0;
    for line in reader("src/input.txt").lines() {
        let line = line.unwrap();

        let opp = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        let opp = RPS::of(opp);
        let me = RPS::of(me);

        let outcome = resolve(&me, &opp);

        let mut score = outcome.score();
        score += match &me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        };

        total_score += score;
        println!("{:?} {:?} {:?} {:?}", opp, me, outcome, score);
    }

    println!("Total: {:?}", total_score);
}

fn part2() {
    println!("Hello, world!");
    let mut total_score = 0;
    for line in reader("src/input.txt").lines() {
        let line = line.unwrap();

        let opp = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        let opp = RPS::of(opp);
        let outcome = Outcome::of(me);

        let me = resolve_my_play(&outcome, &opp);

        let mut score = outcome.score();
        score += match &me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        };

        total_score += score;
        println!("{:?} {:?} {:?} {:?}", opp, me, outcome, score);
    }

    println!("Total: {:?}", total_score);
}

fn main() {
    part2();
}
