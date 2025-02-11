extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<isize> {
    Some(input.chars().map(floor_diff).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += floor_diff(c);
        if floor < 0 {
            return Some(i + 1);
        }
    }
    None
}

fn floor_diff(c: char) -> isize {
    match c {
        '(' => 1,
        ')' => -1,
        _ => panic!("Invalid character in input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_0() {
        let result = part_one("(())");
        assert_eq!(result, Some(0));

        let result = part_one("()()");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one("(((");
        assert_eq!(result, Some(3));

        let result = part_one("(()(()(");
        assert_eq!(result, Some(3));

        let result = part_one("))(((((");
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_neg1() {
        let result = part_one("())");
        assert_eq!(result, Some(-1));

        let result = part_one("))(");
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_part_one_neg3() {
        let result = part_one(")))");
        assert_eq!(result, Some(-3));

        let result = part_one(")())())");
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(")");
        assert_eq!(result, Some(1));

        let result = part_two("()())");
        assert_eq!(result, Some(5));
    }
}
