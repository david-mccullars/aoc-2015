extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    mine(input.trim(), |b0, b1, b2| {
        b0 == 0 && b1 == 0 && (b2 >> 4) == 0
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    mine(input.trim(), |b0, b1, b2| b0 == 0 && b1 == 0 && b2 == 0)
}

fn mine(prefix: &str, is_valid: fn(u8, u8, u8) -> bool) -> Option<usize> {
    let mut context = md5::Context::new();
    context.consume(prefix);
    (1..).find(|n| {
        let mut c2 = context.clone();
        c2.consume(n.to_string());
        let digest = c2.compute();
        let bytes = &digest.as_slice()[0..=3];
        is_valid(bytes[0], bytes[1], bytes[2])
    })
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
