extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(23);

#[derive(Debug)]
enum Ins {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

use Ins::*;

pub fn part_one(input: &str) -> Option<usize> {
    run(input, [0, 0])
}

pub fn part_two(input: &str) -> Option<usize> {
    run(input, [1, 0])
}

fn run(input: &str, mut registers: [usize; 2]) -> Option<usize> {
    let reg = parser!(char_of("ab"));
    let instructions = parser!(lines({
        "hlf " r:reg => Hlf(r),
        "tpl " r:reg => Tpl(r),
        "inc " r:reg => Inc(r),
        "jmp " offset:isize => Jmp(offset),
        "jie " r:reg ", " offset:isize => Jie(r, offset),
        "jio " r:reg ", " offset:isize => Jio(r, offset),
    }))
    .parse(input)
    .unwrap();

    let mut ptr: isize = 0;
    while ptr >= 0 && (ptr as usize) < instructions.len() {
        match instructions[ptr as usize] {
            Hlf(r) => {
                registers[r] >>= 1;
            }
            Tpl(r) => {
                registers[r] *= 3;
            }
            Inc(r) => {
                registers[r] += 1;
            }
            Jmp(v) => {
                ptr += v - 1;
            }
            Jie(r, v) if registers[r] % 2 == 0 => {
                ptr += v - 1;
            }
            Jio(r, v) if registers[r] == 1 => {
                ptr += v - 1;
            }
            _ => {}
        }
        ptr += 1;
    }
    Some(registers[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
