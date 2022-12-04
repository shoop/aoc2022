use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::vec::Vec;

#[derive(Debug)]
struct AssignmentPair {
    first: Range<i32>,
    second: Range<i32>,
}

impl From<&String> for AssignmentPair {
    fn from(str: &String) -> Self {
        let mut assignments = str.split(",");
        let mut first_limits = assignments
            .next()
            .expect("First limits not found")
            .split("-");
        let first_start = first_limits
            .next()
            .expect("Invalid first start")
            .parse::<i32>()
            .expect("First start not a number");
        let first_end = first_limits
            .next()
            .expect("Invalid first end")
            .parse::<i32>()
            .expect("First end not a number");
        let mut second_limits = assignments
            .next()
            .expect("Second limits not found")
            .split("-");
        let second_start = second_limits
            .next()
            .expect("Invalid second start")
            .parse::<i32>()
            .expect("Second start not a number");
        let second_end = second_limits
            .next()
            .expect("Invalid second end")
            .parse::<i32>()
            .expect("Second end not a number");
        AssignmentPair {
            first: Range {
                start: first_start,
                end: first_end + 1,
            },
            second: Range {
                start: second_start,
                end: second_end + 1,
            },
        }
    }
}

impl AssignmentPair {
    fn has_dubious_assignment(&self) -> bool {
        (self.first.start >= self.second.start && self.first.end <= self.second.end)
            || (self.second.start >= self.first.start && self.second.end <= self.first.end)
    }

    fn has_any_overlap(&self) -> bool {
        self.first.contains(&self.second.start)
            || self.first.contains(&(self.second.end - 1))
            || self.second.contains(&self.first.start)
            || self.second.contains(&(self.first.end - 1))
    }
}

fn star_one(lines: &Vec<String>) -> i32 {
    let mut total_dubious_assignments: i32 = 0;
    for line in lines {
        let ap = AssignmentPair::from(line);
        if ap.has_dubious_assignment() {
            total_dubious_assignments += 1;
        }
    }

    total_dubious_assignments
}

fn star_two(lines: &Vec<String>) -> i32 {
    let mut total_overlapping_assignments: i32 = 0;
    for line in lines {
        let ap = AssignmentPair::from(line);
        if ap.has_any_overlap() {
            total_overlapping_assignments += 1;
        }
    }

    total_overlapping_assignments
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
    static TEST_DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 4);
    }
}
