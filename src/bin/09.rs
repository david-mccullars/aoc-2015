extern crate advent_of_code;
extern crate itertools;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    path_distances(input).min()
}

pub fn part_two(input: &str) -> Option<usize> {
    path_distances(input).max()
}

fn path_distances(input: &str) -> impl Iterator<Item = usize> {
    let distances = parser!(lines(string(alpha+) " to " string(alpha+) " = " usize))
        .parse(input)
        .unwrap();

    let mut ids = IdMap::new();
    let mut map = HashMap::new();
    for (a, b, d) in distances {
        map.insert(sort((ids.id(&a), ids.id(&b))), d);
    }

    ids.ids()
        .permutations(ids.ids().len())
        .filter(|path| path[0] < path[path.len() - 1])
        .map(move |path| {
            path.into_iter()
                .tuple_windows()
                .map(|pair| map.get(&sort(pair)).unwrap())
                .sum()
        })
}

fn sort((a, b): (usize, usize)) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
