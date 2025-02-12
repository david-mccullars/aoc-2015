extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(6);

enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(count_lights(input, |op, current| match op {
        Op::TurnOn => 1,
        Op::TurnOff => 0,
        Op::Toggle => {
            if current > 0 {
                0
            } else {
                1
            }
        }
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(count_lights(input, |op, current| match op {
        Op::TurnOn => current + 1,
        Op::TurnOff => {
            if current == 0 {
                0
            } else {
                current - 1
            }
        }
        Op::Toggle => current + 2,
    }))
}

fn count_lights(input: &str, apply_rules: fn(&Op, usize) -> usize) -> usize {
    let pos = parser!({ x:usize "," y:usize => (x, y) });
    let instructions = parser!(lines({
        "turn on " a:pos " through " b:pos => (a, b, Op::TurnOn),
        "turn off " a:pos " through " b:pos => (a, b, Op::TurnOff),
        "toggle " a:pos " through " b:pos => (a, b, Op::Toggle),
    }))
    .parse(input)
    .unwrap();

    let mut lights = vec![];
    for _ in 0..1000 {
        lights.push([0; 1000]);
    }

    for (a, b, op) in instructions {
        for row in lights.iter_mut().take(b.1 + 1).skip(a.1) {
            for v in row.iter_mut().take(b.0 + 1).skip(a.0) {
                *v = apply_rules(&op, *v);
            }
        }
    }

    lights.iter().map(|row| row.iter().sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_000_000 - 1_000 - 4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_000_000 + 2_000 - 4));
    }
}
