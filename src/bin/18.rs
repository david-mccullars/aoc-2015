extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::convert::TryInto;

advent_of_code::solution!(18);

#[cfg(test)]
const STEPS1: usize = 4;
#[cfg(not(test))]
const STEPS1: usize = 100;

#[cfg(test)]
const STEPS2: usize = 5;
#[cfg(not(test))]
const STEPS2: usize = 100;

pub fn part_one(input: &str) -> Option<usize> {
    Some(Lights::new(input).on_after_n_steps(STEPS1))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        Lights::new(input)
            .with_stuck_corners()
            .on_after_n_steps(STEPS2),
    )
}

#[derive(Debug, Clone)]
struct Lights {
    on: Vec<Vec<bool>>,
    corners_stuck: bool,
}

impl Lights {
    fn new(input: &str) -> Self {
        let mut grid = parser!(grid_of(".#")).parse(input).unwrap();
        let mut on = vec![];
        for _ in 0..=grid.bounds.1 {
            on.push(vec![false; (grid.bounds.0 + 1).try_into().unwrap()]);
        }
        for (x, y) in grid.take_all('#') {
            on[y as usize][x as usize] = true;
        }

        Self {
            on,
            corners_stuck: false,
        }
    }

    fn with_stuck_corners(mut self) -> Self {
        self.corners_stuck = true;
        self.turn_on_corners();
        self
    }

    fn on_after_n_steps(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.step();
        }
        self.on
            .iter()
            .map(|row| row.iter().filter(|on| **on).count())
            .sum()
    }

    fn step(&mut self) {
        let mut on2 = self.on.clone();
        for (y, row) in self.on.iter().enumerate() {
            for (x, is_on) in row.iter().enumerate() {
                let neighbors_on = self.neighbors_on(x, y);
                on2[y][x] = if *is_on {
                    neighbors_on == 2 || neighbors_on == 3
                } else {
                    neighbors_on == 3
                };
            }
        }
        self.on = on2;
        if self.corners_stuck {
            self.turn_on_corners();
        }
    }

    fn neighbors_on(&self, cx: usize, cy: usize) -> usize {
        let mut on = 0;
        for x in cx.saturating_sub(1)..=(cx + 1) {
            for y in cy.saturating_sub(1)..=(cy + 1) {
                if (x != cx || y != cy)
                    && y < self.on.len()
                    && x < self.on[y].len()
                    && self.on[y][x]
                {
                    on += 1;
                }
            }
        }
        on
    }

    fn turn_on_corners(&mut self) {
        let ymax = self.on.len() - 1;
        let xmax = self.on[0].len() - 1;
        self.on[0][0] = true;
        self.on[0][xmax] = true;
        self.on[ymax][0] = true;
        self.on[ymax][xmax] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }
}
