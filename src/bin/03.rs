extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    Some(visit::<1>(input))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(visit::<2>(input))
}

fn visit<const N: usize>(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut current = [Pos::default(); N];
    visited.insert(Pos::default());

    for (i, c) in input.chars().enumerate() {
        let dir = Direction::from_char(c);
        current[i % N] = dir.forward_from(&current[i % N]);
        visited.insert(current[i % N]);
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(">");
        assert_eq!(result, Some(2));

        let result = part_one("^>v<");
        assert_eq!(result, Some(4));

        let result = part_one("^v^v^v^v^v");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("^v");
        assert_eq!(result, Some(3));

        let result = part_two("^>v<");
        assert_eq!(result, Some(3));

        let result = part_two("^v^v^v^v^v");
        assert_eq!(result, Some(11));
    }
}
