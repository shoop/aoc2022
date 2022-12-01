use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn star_one(lines: &Vec<String>) -> isize {
    let mut largest: isize = -1;
    let mut current: isize = 0;

    for line in lines.iter() {
        if line.is_empty() {
            if current > largest {
                largest = current;
            }
            current = 0;
            continue;
        }

        let cur = line
            .parse::<isize>()
            .expect(&format!("Invalid number: {}", &line));
        current += cur;
    }

    if current > largest {
        largest = current;
    }

    largest
}

fn star_two(lines: &Vec<String>) -> isize {
    let mut largest: [isize; 3] = [-1; 3];
    let mut current: isize = 0;

    for line in lines.iter() {
        if line.is_empty() {
            if current > largest[0] {
                largest[2] = largest[1];
                largest[1] = largest[0];
                largest[0] = current;
            } else if current > largest[1] {
                largest[2] = largest[1];
                largest[1] = current;
            } else if current > largest[2] {
                largest[2] = current;
            }
            current = 0;
            continue;
        }

        let cur = line
            .parse::<isize>()
            .expect(&format!("Invalid number: {}", &line));
        current += cur;
    }

    if current > largest[0] {
        largest[2] = largest[1];
        largest[1] = largest[0];
        largest[0] = current;
    } else if current > largest[1] {
        largest[2] = largest[1];
        largest[1] = current;
    } else if current > largest[2] {
        largest[2] = current;
    }

    largest[0] + largest[1] + largest[2]
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
    static TEST_DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, 24000);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, 45000);
    }
}
