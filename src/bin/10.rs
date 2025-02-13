extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    Some(look_and_say_size_after(input, 40))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(look_and_say_size_after(input, 50))
}

fn look_and_say_size_after(input: &str, n: usize) -> usize {
    let mut digits = parser!(line(digit+)).parse(input).unwrap();
    for _ in 0..n {
        digits = look_and_say(digits);
    }
    digits.len()
}

fn look_and_say(digits: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut seq = vec![];
    for d in digits {
        if i > 0 && seq[i - 1] == d {
            seq[i - 2] += 1;
        } else {
            seq.push(1);
            seq.push(d);
            i += 2;
        }
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(237746));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3369156));
    }
}
