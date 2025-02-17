extern crate advent_of_code;
extern crate nalgebra;

#[allow(unused_imports)]
use advent_of_code::*;
use nalgebra::DMatrix;

advent_of_code::solution!(15);

const MAX_TSP: i64 = 100;

pub fn part_one(input: &str) -> Option<i64> {
    best_score(input, None)
}

pub fn part_two(input: &str) -> Option<i64> {
    best_score(input, Some(500))
}

fn best_score(input: &str, calories: Option<i64>) -> Option<i64> {
    let ingr = parser!(lines({
        string(alpha+)
        ": capacity " i64
        ", durability " i64
        ", flavor " i64
        ", texture " i64
        ", calories " i64
    }))
    .parse(input)
    .unwrap();

    let mut data: Vec<i64> = vec![];
    for (_, c, d, f, t, k) in &ingr {
        data.extend([c, d, f, t, k]);
    }
    let matrix = DMatrix::from_column_slice(5, ingr.len(), &data);

    allocations(ingr.len(), MAX_TSP)
        .filter_map(|allocation| {
            let product = matrix.clone() * DMatrix::from_column_slice(ingr.len(), 1, &allocation);
            let scores = product.column(0);

            if calories.is_some() && calories != Some(scores[4]) {
                None
            } else {
                scores
                    .iter()
                    .all(|i| *i > 0)
                    .then(|| scores.iter().take(4).product::<i64>())
            }
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62842880));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57600000));
    }
}
