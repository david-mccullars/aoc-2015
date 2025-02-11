extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let vowels = line
                    .chars()
                    .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                    .count();
                if vowels < 3 {
                    return false;
                }

                let mut has_double = false;
                for w in line.as_bytes().windows(2) {
                    if w[0] == w[1] {
                        has_double = true;
                    }
                    if w == [b'a', b'b']
                        || w == [b'c', b'd']
                        || w == [b'p', b'q']
                        || w == [b'x', b'y']
                    {
                        return false;
                    }
                }
                has_double
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let mut pairs = HashMap::new();
                for (i, e) in line.as_bytes().windows(2).enumerate() {
                    pairs.entry(e).or_insert_with(std::vec::Vec::new).push(i);
                }
                let has_valid_pair = pairs
                    .values()
                    .any(|v| v.len() > 2 || (v.len() == 2 && v[0] + 1 < v[1]));

                let chars = line.as_bytes();
                let has_valid_repeat = (2..chars.len()).any(|i| chars[i - 2] == chars[i]);

                has_valid_pair && has_valid_repeat
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
