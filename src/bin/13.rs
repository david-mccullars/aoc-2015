extern crate advent_of_code;
extern crate itertools;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<isize> {
    SeatPreferences::parse(input).best_arrangement()
}

pub fn part_two(input: &str) -> Option<isize> {
    SeatPreferences::parse(input).add_me().best_arrangement()
}

struct SeatPreferences {
    ids: IdMap<String>,
    deltas: HashMap<(usize, usize), isize>,
}

impl SeatPreferences {
    fn parse(input: &str) -> Self {
        let gain_or_lose = parser!({
            "gain " n:isize => n,
            "lose " n:isize => -n,
        });
        let info = parser!(
            lines(string(alpha+) " would " gain_or_lose
            " happiness units by sitting next to " string(alpha+) "."
        ))
        .parse(input)
        .unwrap();

        let mut ids = IdMap::new();
        let mut deltas = HashMap::new();
        for (a, delta, b) in info {
            let (ai, bi) = (ids.id(&a), ids.id(&b));
            deltas.insert((ai, bi), delta);
        }

        Self { ids, deltas }
    }

    fn add_me(mut self) -> Self {
        let me = self.ids.id(&"ME".to_string());
        for id in self.ids.ids() {
            self.deltas.insert((id, me), 0);
            self.deltas.insert((me, id), 0);
        }
        self
    }

    fn best_arrangement(&self) -> Option<isize> {
        self.ids
            .ids()
            .permutations(self.ids.ids().len())
            .map(|arrangement| {
                // Optimization - WLOG, assume first person is always first
                if arrangement[0] > 0 {
                    return 0;
                }
                let mut arrangement = arrangement.clone();
                arrangement.push(arrangement[0]);
                arrangement
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| {
                        self.deltas.get(&(a, b)).unwrap() + self.deltas.get(&(b, a)).unwrap()
                    })
                    .sum::<isize>()
            })
            .max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
