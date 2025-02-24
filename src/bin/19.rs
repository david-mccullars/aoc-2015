extern crate advent_of_code;
extern crate regex;

#[allow(unused_imports)]
use advent_of_code::*;
use regex::Regex;
use std::collections::HashSet;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (transitions, medicine) = parse(input);

    let mut strings = HashSet::new();
    for (a, b) in &transitions {
        let mut i = 0;
        while let Some(j) = medicine[i..].find(a) {
            i += j + a.len();
            strings.insert(format!(
                "{}{}{}",
                &medicine[0..(i - a.len())],
                b,
                &medicine[i..]
            ));
        }
    }
    Some(strings.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, medicine) = parse(input);

    let mut total = 0;
    let mut rn_ar = 0;
    let mut y = 0;

    let token_re = Regex::new(r"[A-Z][a-z]*").unwrap();
    for token in token_re.find_iter(&medicine) {
        total += 1;
        match token.as_str() {
            "Rn" => {
                rn_ar += 1;
            }
            "Ar" => {
                rn_ar += 1;
            }
            "Y" => {
                y += 2;
            }
            _ => {}
        }
    }
    Some(total - rn_ar - y - 1)
}

fn parse(input: &str) -> (Vec<(String, String)>, String) {
    parser!(
        section(lines(string(alpha+) " => " string(alpha+)))
        section(line(string(alpha+)))
    )
    .parse(input)
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
