extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(16);

lazy_static::lazy_static! {
    static ref FOUND: HashMap<&'static str, usize> = {
        let mut h = HashMap::new();
        h.insert("children", 3);
        h.insert("cats", 7);
        h.insert("samoyeds", 2);
        h.insert("pomeranians", 3);
        h.insert("akitas", 0);
        h.insert("vizslas", 0);
        h.insert("goldfish", 5);
        h.insert("trees", 3);
        h.insert("cars", 2);
        h.insert("perfumes", 1);
        h
    };
}

pub fn part_one(input: &str) -> Option<usize> {
    find_sue(input, |(name, qty)| FOUND.get(name.as_str()) == Some(qty))
}

pub fn part_two(input: &str) -> Option<usize> {
    find_sue(input, |(name, qty)| {
        if let Some(qty2) = FOUND.get(name.as_str()) {
            match name.as_str() {
                "cats" | "trees" => qty > qty2,
                "pomeranians" | "goldfish" => qty < qty2,
                _ => qty == qty2,
            }
        } else {
            false
        }
    })
}

fn find_sue(input: &str, finder: fn((&String, &usize)) -> bool) -> Option<usize> {
    let clue = parser!(string(alpha+) ": " usize);
    let sues = parser!(lines("Sue " usize ": " hash_map(repeat_sep(clue, ", "))))
        .parse(input)
        .unwrap();

    sues.into_iter()
        .find(|(_, remembered)| remembered.iter().all(finder))
        .map(|(sue, _)| sue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(103));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }
}
