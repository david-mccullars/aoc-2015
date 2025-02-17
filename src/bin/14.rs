extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(14);

#[cfg(test)]
const TIME: u32 = 1_000;
#[cfg(not(test))]
const TIME: u32 = 2_503;

pub fn part_one(input: &str) -> Option<f64> {
    parse(input)
        .iter()
        .map(|deer| distance(deer, TIME))
        .max_by(|a, b| a.total_cmp(b))
}

pub fn part_two(input: &str) -> Option<usize> {
    let deer = parse(input);
    let mut scores = vec![0; deer.len()];

    for time in 1..=TIME {
        let distances: Vec<_> = deer.iter().map(|deer| distance(deer, time)).collect();
        let max = distances.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        for (i, d) in distances.iter().enumerate() {
            if max == d {
                scores[i] += 1;
            }
        }
    }
    scores.into_iter().max()
}

type Deer = (String, f64, u32, u32);

fn parse(input: &str) -> Vec<Deer> {
    parser!(lines(
        string(alpha+) " can fly " f64 " km/s for "
        u32 " seconds, but then must rest for " u32 " seconds."
    ))
    .parse(input)
    .unwrap()
}

fn distance((_, v, v_t, r_t): &Deer, time: u32) -> f64 {
    let n = time / (v_t + r_t);
    let x_t = time % (v_t + r_t);
    let t = f64::from(n * v_t + x_t.min(*v_t));
    v * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1120.0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(689));
    }
}
