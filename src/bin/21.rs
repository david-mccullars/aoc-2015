extern crate advent_of_code;
extern crate itertools;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(21);

type Stat = (usize, usize, usize);

const WEAPONS: [Stat; 5] = [
    (8, 4, 0),  // Dagger
    (10, 5, 0), // Shortsword
    (25, 6, 0), // Warhammer
    (40, 7, 0), // Longsword
    (74, 8, 0), // Greataxe
];

const ARMOR: [Stat; 6] = [
    (0, 0, 0),   // (None)
    (13, 0, 1),  // Leather
    (31, 0, 2),  // Chainmail
    (53, 0, 3),  // Splintmail
    (75, 0, 4),  // Bandedmail
    (102, 0, 5), // Platemail
];

const SINGLE_RINGS: [Stat; 7] = [
    (0, 0, 0),   // (None)
    (25, 1, 0),  // Damage +1
    (50, 2, 0),  // Damage +2
    (100, 3, 0), // Damage +3
    (20, 0, 1),  // Defense +1
    (40, 0, 2),  // Defense +2
    (80, 0, 3),  // Defense +3
];

lazy_static::lazy_static! {
    static ref POSSIBLE_PURCHASES: Vec<Stat> = {
        let two_rings: Vec<_>= SINGLE_RINGS.iter().tuple_combinations().map(|((c1, d1, a1), (c2, d2, a2))|
            (c1 + c2, d1 + d2, a1 + a2)
        ).collect();
        let any_rings: Vec<_> = SINGLE_RINGS.iter().chain(two_rings.iter()).collect();

        WEAPONS.iter().cartesian_product(ARMOR.iter()).cartesian_product(any_rings.iter()).map(|((a, b), c)|
            (a.0 + b.0 + c.0, a.1 + b.1 + c.1, a.2 + b.2 + c.2)
        ).collect()
    };
}

pub fn part_one(input: &str) -> Option<usize> {
    let monster = monster(input);
    POSSIBLE_PURCHASES
        .iter()
        .filter(|(_, damage, armor)| do_i_win((100, *damage, *armor), monster))
        .map(|(cost, _, _)| *cost)
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let monster = monster(input);
    POSSIBLE_PURCHASES
        .iter()
        .filter(|(_, damage, armor)| !do_i_win((100, *damage, *armor), monster))
        .map(|(cost, _, _)| *cost)
        .max()
}

fn monster(input: &str) -> Stat {
    parser!(
        line("Hit Points: " usize)
        line("Damage: " usize)
        line("Armor: " usize)
    )
    .parse(input.trim())
    .unwrap()
}

fn do_i_win(mut me: Stat, mut monster: Stat) -> bool {
    loop {
        monster.0 = monster
            .0
            .saturating_sub(me.1.saturating_sub(monster.2).max(1));
        if monster.0 == 0 {
            return true;
        }
        me.0 = me.0.saturating_sub(monster.1.saturating_sub(me.2).max(1));
        if me.0 == 0 {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
