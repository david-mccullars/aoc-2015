extern crate advent_of_code;
extern crate regex;

#[allow(unused_imports)]
use advent_of_code::*;
use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let unescaped_len: usize = input.lines().map(unescape).sum();
    Some(input.len() - unescaped_len - input.lines().count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let escaped_len: usize = input.lines().map(escape).sum();
    Some(escaped_len - input.len() + input.lines().count())
}

fn unescape(line: &str) -> usize {
    let hex_re = Regex::new(r#"\\x[0-9a-f]{2}"#).unwrap();
    let unescaped = hex_re
        .replace_all(line, "X") // Any one-single character will do
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");
    unescaped.len() - 2
}

fn escape(line: &str) -> usize {
    2 + line.replace("\\", "\\\\").replace("\"", "\\\"").len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
