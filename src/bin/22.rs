extern crate advent_of_code;
extern crate pathfinding;

#[allow(unused_imports)]
use advent_of_code::*;
use pathfinding::directed::dijkstra::dijkstra;

advent_of_code::solution!(22);

#[cfg(test)]
const PLAYER_HIT_POINTS: isize = 10;
#[cfg(not(test))]
const PLAYER_HIT_POINTS: isize = 50;
#[cfg(test)]
const PLAYER_MANA: isize = 250;
#[cfg(not(test))]
const PLAYER_MANA: isize = 500;

#[derive(Clone, Copy, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

fn spell_cost(spell: &Spell) -> isize {
    match spell {
        MagicMissile => 53,
        Drain => 73,
        Shield => 113,
        Poison => 173,
        Recharge => 229,
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct SpellsInEffect {
    shield: usize,
    poison: usize,
    recharge: usize,
}

impl SpellsInEffect {
    fn decay(&mut self) {
        self.shield = self.shield.saturating_sub(1);
        self.poison = self.poison.saturating_sub(1);
        self.recharge = self.recharge.saturating_sub(1);
    }
}

use Spell::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct State {
    monster_hit_points: isize,
    player_hit_points: isize,
    player_mana: isize,
    spells_in_effect: SpellsInEffect,
}

impl State {
    fn is_win(&self) -> bool {
        self.monster_hit_points <= 0
    }

    fn is_lose(&self) -> bool {
        self.player_hit_points <= 0
    }

    fn is_spell_in_effect(&self, spell: &Spell) -> bool {
        (match spell {
            Shield => self.spells_in_effect.shield,
            Poison => self.spells_in_effect.poison,
            Recharge => self.spells_in_effect.recharge,
            _ => 0,
        }) > 0
    }

    fn turn(&self, spell: &Spell, monster_damage: isize, hard_mode: bool) -> Option<(Self, isize)> {
        let mut state = self.clone();
        let cost = spell_cost(spell);

        // Player Turn

        if hard_mode {
            state.player_hit_points -= 1;
            if state.is_lose() {
                return None;
            }
        }

        let _ = state.apply_spells_in_effect();
        if state.is_win() {
            return Some((state, 0));
        } else if state.is_spell_in_effect(spell) || state.player_mana < cost {
            return None;
        }

        match spell {
            MagicMissile => {
                state.monster_hit_points -= 4;
            }
            Drain => {
                state.monster_hit_points -= 2;
                state.player_hit_points += 2;
            }
            Shield => {
                state.spells_in_effect.shield = 6;
            }
            Poison => {
                state.spells_in_effect.poison = 6;
            }
            Recharge => {
                state.spells_in_effect.recharge = 5;
            }
        }
        state.player_mana -= cost;

        // Boss Turn

        let player_armor = state.apply_spells_in_effect();
        if !state.is_win() {
            state.player_hit_points -= (monster_damage - player_armor).max(1);
            if state.is_lose() {
                return None;
            }
        }

        Some((state, cost))
    }

    fn apply_spells_in_effect(&mut self) -> isize {
        let player_armor = if self.is_spell_in_effect(&Shield) {
            7
        } else {
            0
        };
        if self.is_spell_in_effect(&Poison) {
            self.monster_hit_points -= 3;
        }
        if self.is_spell_in_effect(&Recharge) {
            self.player_mana += 101;
        }
        self.spells_in_effect.decay();
        player_armor
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    least_mana(input, false)
}

pub fn part_two(input: &str) -> Option<isize> {
    least_mana(input, true)
}

fn least_mana(input: &str, hard_mode: bool) -> Option<isize> {
    let (monster_hit_points, monster_damage) = monster(input);
    let initial_state = State {
        monster_hit_points,
        player_hit_points: PLAYER_HIT_POINTS,
        player_mana: PLAYER_MANA,
        ..Default::default()
    };

    let successors = |state: &State| {
        [Recharge, Poison, Shield, Drain, MagicMissile]
            .iter()
            .filter_map(|spell| state.turn(spell, monster_damage, hard_mode))
            .collect::<Vec<_>>()
    };
    let success = |state: &State| state.is_win();

    dijkstra(&initial_state, successors, success).map(|(_, cost)| cost)
}

fn monster(input: &str) -> (isize, isize) {
    parser!(
        line("Hit Points: " isize)
        line("Damage: " isize)
    )
    .parse(input.trim())
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(641));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
