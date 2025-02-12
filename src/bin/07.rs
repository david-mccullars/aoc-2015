extern crate advent_of_code;
extern crate memoize;

#[allow(unused_imports)]
use advent_of_code::*;
use memoize::memoize;
use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u16> {
    let wiring = parse_wiring(input);
    Some(value(&wiring, "a".to_string()))
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut wiring = parse_wiring(input);
    let a = value(&wiring, "a".to_string());

    memoized_flush_value();
    wiring.insert("b".to_string(), Source::Raw(Signal::Value(a)));
    Some(value(&wiring, "a".to_string()))
}

#[derive(Eq, PartialEq)]
enum Signal {
    Value(u16),
    Wire(String),
}

#[derive(Eq, PartialEq)]
enum Source {
    Raw(Signal),
    Not(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    Lshift(Signal, Signal),
    Rshift(Signal, Signal),
}

type Wiring = HashMap<String, Source>;

fn parse_wiring(input: &str) -> Wiring {
    let signal = parser!({
        v:u16 => Signal::Value(v),
        v:string(alpha+) => Signal::Wire(v),
    });

    let source = parser!({
        v:signal => Source::Raw(v),
        "NOT " v:signal => Source::Not(v),
        a:signal " AND " b:signal => Source::And(a, b),
        a:signal " OR " b:signal => Source::Or(a, b),
        a:signal " LSHIFT " b:signal => Source::Lshift(a, b),
        a:signal " RSHIFT " b:signal => Source::Rshift(a, b),
    });

    parser!(hash_map(lines({
        src:source " -> " wire:string(alpha+) => (wire, src)
    })))
    .parse(input)
    .unwrap()
}

#[memoize(Ignore: wiring)]
fn value(wiring: &Wiring, wire: String) -> u16 {
    let val = |signal: &Signal| match signal {
        Signal::Value(v) => *v,
        Signal::Wire(w) => value(wiring, w.clone()),
    };

    match wiring.get(&wire).unwrap() {
        Source::Raw(s) => val(s),
        Source::Not(s) => !val(s),
        Source::And(s1, s2) => val(s1) & val(s2),
        Source::Or(s1, s2) => val(s1) | val(s2),
        Source::Lshift(s1, s2) => val(s1) << val(s2),
        Source::Rshift(s1, s2) => val(s1) >> val(s2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65517));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(49156));
    }
}
