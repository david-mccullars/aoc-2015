extern crate advent_of_code;
extern crate itertools;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<usize> {
    sleigh_balance(input, 3)
}

pub fn part_two(input: &str) -> Option<usize> {
    sleigh_balance(input, 4)
}

fn sleigh_balance(input: &str, compartments: usize) -> Option<usize> {
    let mut packages = parser!(lines(usize)).parse(input).unwrap();
    packages.sort_by(|a, b| b.cmp(a));

    let total: usize = packages.iter().sum();
    assert_eq!(total % compartments, 0);
    let per_side = total / compartments;

    let min_packages = packages
        .iter()
        .scan(0, |sum, x| {
            *sum += *x;
            Some(*sum)
        })
        .position(|sum| sum > per_side)
        .unwrap()
        + 1;

    let mut valid_groups = vec![];
    for n in min_packages.. {
        valid_groups = packages
            .iter()
            .combinations(n)
            .filter(|subset| subset.iter().copied().sum::<usize>() == per_side)
            .collect();
        if !valid_groups.is_empty() {
            break;
        }
    }

    valid_groups.into_iter().map(quantum_entanglement).min()
}

fn quantum_entanglement(packages: Vec<&usize>) -> usize {
    packages.into_iter().product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }
}
