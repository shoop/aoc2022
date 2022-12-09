use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn find_distinct_run(line: &String, len: usize) -> usize {
    for i in 0..(line.len() - len - 1) {
        let signal = &line.as_bytes()[i..i+len];
        let mut equal: bool = false;
        for cmp in 0..len {
            for tgt in cmp+1..len {
                if signal[cmp] == signal[tgt] {
                    equal = true;
                    break;
                }
            }
            if equal == true {
                break;
            }
        }

        if !equal {
            return i + len;
        }
    }

    panic!("No signal of len {} found in {}", len, line);
}

fn star_one(lines: &Vec<String>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for line in lines {
        result.push(find_distinct_run(line, 4));
    }

    result
}

fn star_two(lines: &Vec<String>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for line in lines {
        result.push(find_distinct_run(line, 14));
    }

    result
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .collect();

    let ans = star_one(&lines);
    println!("Star one: {}", ans[0]);

    let ans = star_two(&lines);
    println!("Star two: {}", ans[0]);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_star_one() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_one(&lines);
        assert_eq!(ans, [7, 5, 6, 10, 11]);
    }

    #[test]
    fn test_star_two() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let ans = super::star_two(&lines);
        assert_eq!(ans, [19, 23, 23, 29, 26]);
    }
}
