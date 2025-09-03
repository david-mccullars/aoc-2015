extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    md5_search(input, |d| d[0] == 0 && d[1] == 0 && (d[2] >> 4) == 0).next()
}

pub fn part_two(input: &str) -> Option<usize> {
    md5_search(input, |d| d[0] == 0 && d[1] == 0 && d[2] == 0).next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1048970));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6742839));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(5714438));
    }
}
