extern crate advent_of_code;
extern crate json;

#[allow(unused_imports)]
use advent_of_code::*;
use json::JsonValue;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    Some(sum(&json::parse(input).unwrap(), None))
}

pub fn part_two(input: &str) -> Option<i64> {
    let red = "red".into();
    Some(sum(&json::parse(input).unwrap(), Some(&red)))
}

fn sum(obj: &JsonValue, ignore: Option<&JsonValue>) -> i64 {
    if has_ignore(obj, ignore) {
        return 0;
    }
    match obj {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Object(o) => o.iter().map(|(_, v)| sum(v, ignore)).sum(),
        JsonValue::Array(a) => a.iter().map(|v| sum(v, ignore)).sum(),
        _ => 0,
    }
}

fn has_ignore(obj: &JsonValue, ignore: Option<&JsonValue>) -> bool {
    if let JsonValue::Object(o) = obj {
        if let Some(i) = ignore {
            return o.iter().any(|(_, v)| v == i);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(16));
    }
}
