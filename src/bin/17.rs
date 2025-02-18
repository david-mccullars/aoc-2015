extern crate advent_of_code;
extern crate itertools;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(17);

#[cfg(test)]
const LITERS: usize = 25;
#[cfg(not(test))]
const LITERS: usize = 150;

pub fn part_one(input: &str) -> Option<usize> {
    Some(combos_that_fit(input).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    combos_that_fit(input).next()
}

fn combos_that_fit(input: &str) -> impl Iterator<Item = usize> {
    let sizes = parser!(lines(usize)).parse(input).unwrap();
    (1..=sizes.len()).filter_map(move |n| {
        let cnt = sizes
            .iter()
            .combinations(n)
            .filter(|c| c.iter().copied().sum::<usize>() == LITERS)
            .count();
        (cnt > 0).then_some(cnt)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
