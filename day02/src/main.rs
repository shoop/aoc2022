use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

const POINTS_ROCK: i32 = 1;
const POINTS_PAPER: i32 = 2;
const POINTS_SCISSORS: i32 = 3;

const POINTS_LOSS: i32 = 0;
const POINTS_DRAW: i32 = 3;
const POINTS_WIN: i32 = 6;

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Strategy {
    Loss,
    Draw,
    Win,
}

fn calc_game_score(opp: &RPS, my: &RPS) -> i32 {
    match (opp, my)
    {
        (RPS::Rock,     RPS::Rock    ) => POINTS_DRAW + POINTS_ROCK,
        (RPS::Rock,     RPS::Paper   ) => POINTS_WIN  + POINTS_PAPER,
        (RPS::Rock,     RPS::Scissors) => POINTS_LOSS + POINTS_SCISSORS,
        (RPS::Paper,    RPS::Rock    ) => POINTS_LOSS + POINTS_ROCK,
        (RPS::Paper,    RPS::Paper   ) => POINTS_DRAW + POINTS_PAPER,
        (RPS::Paper,    RPS::Scissors) => POINTS_WIN  + POINTS_SCISSORS,
        (RPS::Scissors, RPS::Rock    ) => POINTS_WIN  + POINTS_ROCK,
        (RPS::Scissors, RPS::Paper   ) => POINTS_LOSS + POINTS_PAPER,
        (RPS::Scissors, RPS::Scissors) => POINTS_DRAW + POINTS_SCISSORS,
    }
}

fn star_one(lines: &Vec<String>) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        let mut split = line.split(" ");
        let opp_choice = match split.next().expect("Line too short") {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid input")
        };
        let my_choice = match split.next().expect("Line too short") {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Invalid input")
        };
        score += calc_game_score(&opp_choice, &my_choice);
    }

    score
}

fn star_two(lines: &Vec<String>) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        let mut split = line.split(" ");
        let opp_choice = match split.next().expect("Line too short") {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid input")
        };
        let strategy = match split.next().expect("Line too short") {
            "X" => Strategy::Loss,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("Invalid input")
        };
        let my_choice = match (&opp_choice, &strategy) {
            (&RPS::Rock,     &Strategy::Loss) => RPS::Scissors,
            (&RPS::Rock,     &Strategy::Draw) => RPS::Rock,
            (&RPS::Rock,     &Strategy::Win)  => RPS::Paper,
            (&RPS::Paper,    &Strategy::Loss) => RPS::Rock,
            (&RPS::Paper,    &Strategy::Draw) => RPS::Paper,
            (&RPS::Paper,    &Strategy::Win)  => RPS::Scissors,
            (&RPS::Scissors, &Strategy::Loss) => RPS::Paper,
            (&RPS::Scissors, &Strategy::Draw) => RPS::Scissors,
            (&RPS::Scissors, &Strategy::Win)  => RPS::Rock,
        };
        score += calc_game_score(&opp_choice, &my_choice);
    }

    score
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines);
    println!("Star one: {}", ans);

    let ans = star_two(&lines);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "A Y
B X
C Z";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 15);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 12);
    }
}
