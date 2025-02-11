extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::cmp::min;

advent_of_code::solution!(2);

type Dims = (usize, usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    Some(find(paper_needed, input))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(find(ribbon_needed, input))
}

fn find(required: fn(Dims) -> usize, input: &str) -> usize {
    parser!(lines(usize "x" usize "x" usize))
        .parse(input)
        .unwrap()
        .into_iter()
        .map(required)
        .sum()
}

fn paper_needed((l, w, h): Dims) -> usize {
    let a1 = l * w;
    let a2 = w * h;
    let a3 = l * h;

    2 * (a1 + a2 + a3) + min(min(a1, a2), a3)
}

fn ribbon_needed((l, w, h): Dims) -> usize {
    let p1 = 2 * (l + w);
    let p2 = 2 * (w + h);
    let p3 = 2 * (l + h);

    (l * w * h) + min(min(p1, p2), p3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34 + 14));
    }
}
