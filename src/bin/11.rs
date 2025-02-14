extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<String> {
    next_password(input.trim().to_owned())
}

pub fn part_two(input: &str) -> Option<String> {
    next_password(input.trim().to_owned()).and_then(next_password)
}

fn next_password(old: String) -> Option<String> {
    let mut chars: Vec<_> = old.bytes().collect();
    while inc(&mut chars) {
        if password_valid(&chars) {
            return Some(chars.into_iter().map(|c| (c as char)).collect());
        }
    }
    None
}

fn inc(chars: &mut [u8]) -> bool {
    let mut i = chars.len() - 1;
    while chars[i] == b'z' {
        if i == 0 {
            return false;
        }
        chars[i] = b'a';
        i -= 1;
    }
    chars[i] += 1;
    true
}

fn password_valid(chars: &[u8]) -> bool {
    let has_straight = chars
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    let no_illegal = !chars.iter().any(|c| *c == b'i' || *c == b'o' || *c == b'l');

    has_straight && no_illegal && two_pairs(chars)
}

fn two_pairs(chars: &[u8]) -> bool {
    let mut first = None;
    for (i, w) in chars.windows(2).enumerate() {
        if w[0] == w[1] {
            if let Some((j, c)) = first {
                if c != w[0] && j + 1 < i {
                    return true;
                }
            } else {
                first = Some((i, w[0]));
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("ghjaabcc".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("ghjbbcdd".to_owned()));
    }
}
