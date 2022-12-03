use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct Rucksack {
    priority: i32,
    all: Vec<char>,
    first: Vec<char>,
    second: Vec<char>,
}

impl Rucksack {
    fn find_common_element_in_compartments(&self) -> char {
        *self
            .first
            .iter()
            .filter(|c| self.second.iter().any(|c2| c == &c2))
            .next()
            .unwrap()
    }

    fn priority_of_element(element: char) -> i32 {
        match element as i32 {
            65..=90 => element as i32 - (65 - 27),
            97..=122 => element as i32 - 96,
            _ => panic!("invalid element {}", element),
        }
    }

    fn set_priority(mut self) -> Self {
        let common = self.find_common_element_in_compartments();
        self.priority = Rucksack::priority_of_element(common);
        self
    }

    fn find_common_element_with_group(&self, left: &Rucksack, right: &Rucksack) -> char {
        *self
            .all
            .iter()
            .filter(|c| left.all.iter().any(|cl| c == &cl) && right.all.iter().any(|cr| c == &cr))
            .next()
            .unwrap()
    }
}

impl From<&String> for Rucksack {
    fn from(str: &String) -> Self {
        let compartment_size = str.chars().count() / 2;
        let rs = Rucksack {
            priority: 0,
            all: str.chars().collect(),
            first: str.chars().take(compartment_size).collect(),
            second: str
                .chars()
                .skip(compartment_size)
                .take(compartment_size)
                .collect(),
        };
        rs.set_priority()
    }
}

fn star_one(lines: &Vec<String>) -> i32 {
    let mut total_priority: i32 = 0;
    for line in lines {
        let rs = Rucksack::from(line);
        total_priority += rs.priority;
    }

    total_priority
}

fn star_two(lines: &Vec<String>) -> i32 {
    let mut total_priority: i32 = 0;
    for groups in lines.chunks_exact(3) {
        let group: Vec<Rucksack> = groups
            .into_iter()
            .map(|line| Rucksack::from(line))
            .collect();

        let badge = group[0].find_common_element_with_group(&group[1], &group[2]);
        total_priority += Rucksack::priority_of_element(badge);
    }

    total_priority
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
    static TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 157);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 70);
    }
}
