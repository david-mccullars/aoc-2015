extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(25);

const PREFIX: &str = "To continue, please consult the code grid in the manual.  Enter the code at";

pub fn part_one(input: &str) -> Option<u64> {
    let (r, c) = parser!(line(PREFIX " row " usize ", column " usize "."))
        .parse(input)
        .unwrap();
    let cantor = (r + c - 2) * (r + c - 1) / 2 + c;
    Some((1..cantor).fold(20151125, |hash, _| (hash * 252533) % 33554393))
}

pub fn part_two(_input: &str) -> Option<&str> {
    Some("CLAIM THE FINAL GOLD STAR!!!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1534922));
    }
}
