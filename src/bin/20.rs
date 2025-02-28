extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    presents(input, 10, None)
}

pub fn part_two(input: &str) -> Option<usize> {
    presents(input, 11, Some(50))
}

fn presents(input: &str, presents_per_elf: usize, houses_per_elf: Option<usize>) -> Option<usize> {
    let num: usize = input.trim().parse().unwrap();
    let mut presents = vec![0; num / presents_per_elf];
    let houses_per_elf = houses_per_elf.unwrap_or(num);

    (1..presents.len()).find(|elf| {
        let max = (presents.len() / elf).min(houses_per_elf);
        for i in 1..=max {
            presents[(i * elf) - 1] += elf * presents_per_elf;
        }
        presents[elf - 1] >= num
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(90));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(84));
    }
}
