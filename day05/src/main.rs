use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct Stack {
    crates: VecDeque<char>,
}

#[derive(Debug)]
struct Dock {
    stacks: Vec<Stack>,
}

impl Dock {
    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.crates.front())
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect()
    }
}

impl From<&Vec<String>> for Dock {
    fn from(lines: &Vec<String>) -> Self {
        let mut dock = Dock { stacks: Vec::new() };
        for (lineidx, line) in lines.iter().enumerate() {
            if line.is_empty() {
                break;
            }

            for (i, ch) in line.chars().enumerate() {
                match (i % 4, ch) {
                    (0, '[') => (),
                    (1, ch) if ch != ' ' && !ch.is_numeric() => {
                        while dock.stacks.len() < (i / 4) + 1 {
                            dock.stacks.push(Stack {
                                crates: VecDeque::new(),
                            });
                        }
                        dock.stacks[i / 4].crates.push_back(ch);
                    }
                    (1, ch) if ch.is_numeric() => {
                        break;
                    }
                    (2, ']') => (),
                    (_, ' ') => (),
                    (_, _) => panic!(
                        "Invalid input in line {} position {} char {}",
                        lineidx, i, ch
                    ),
                }
            }
        }

        dock
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl Move {
    fn execute_9000(&self, dock: &mut Dock) {
        for i in 0..self.amount {
            let cr = dock.stacks[self.from]
                .crates
                .pop_front()
                .unwrap_or_else(|| {
                    panic!(
                        "move {} of {} from stack {} failed",
                        i, self.amount, self.from
                    )
                });
            dock.stacks[self.to].crates.push_front(cr);
        }
    }

    fn execute_9001(&self, dock: &mut Dock) {
        let moved: VecDeque<char> =
            VecDeque::from_iter(dock.stacks[self.from].crates.drain(0..self.amount));
        for mv in moved.iter().rev() {
            dock.stacks[self.to].crates.push_front(*mv);
        }
    }
}

impl From<&String> for Move {
    fn from(line: &String) -> Self {
        let mut iter = line.split_whitespace();
        iter.next().expect("move keyword missing");
        let amount = iter
            .next()
            .expect("amount missing")
            .parse::<usize>()
            .expect("invalid amount");
        iter.next().expect("from keyword missing");
        let from = iter
            .next()
            .expect("from missing")
            .parse::<usize>()
            .expect("invalid from");
        iter.next().expect("to keyword missing");
        let to = iter
            .next()
            .expect("to missing")
            .parse::<usize>()
            .expect("invalid to");

        Move {
            from: from - 1,
            to: to - 1,
            amount: amount,
        }
    }
}

fn parse_moves(lines: &Vec<String>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut in_moves: bool = false;
    for line in lines {
        if !in_moves {
            if line.is_empty() {
                in_moves = true;
            }
            continue;
        }

        moves.push(Move::from(line));
    }

    moves
}

fn star_one(lines: &Vec<String>) -> String {
    let mut dock = Dock::from(lines);
    let moves = parse_moves(lines);
    for mv in moves.iter() {
        mv.execute_9000(&mut dock);
    }

    dock.top_crates()
}

fn star_two(lines: &Vec<String>) -> String {
    let mut dock = Dock::from(lines);
    let moves = parse_moves(lines);
    for mv in moves.iter() {
        mv.execute_9001(&mut dock);
    }

    dock.top_crates()
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
    static TEST_DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, "CMZ");
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, "MCD");
    }
}
