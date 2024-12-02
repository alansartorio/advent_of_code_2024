use std::ops::Neg;

use chumsky::{prelude::*, text::newline};
use itertools::Itertools;

use crate::utils::parsers::number;

type Input = Vec<Vec<u64>>;

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    number()
        .separated_by(just(" "))
        .at_least(1)
        .separated_by(newline())
        .at_least(1)
        .then_ignore(end())
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    parser().parse(input).unwrap()
}

fn is_safe_delta(delta: &i64) -> bool {
    (1..=3).contains(delta)
}

fn are_safe_deltas(deltas: &[i64]) -> bool {
    deltas.iter().all(is_safe_delta)
}

fn compute_deltas(line: &[u64]) -> impl Iterator<Item = i64> + '_ {
    line.iter()
        .tuple_windows()
        .map(|(&a, &b)| b as i64 - a as i64)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|line| {
            let mut deltas = compute_deltas(line).collect_vec();

            if deltas[0] < 0 {
                for delta in deltas.iter_mut() {
                    *delta = -*delta;
                }
            }

            are_safe_deltas(&deltas)
        })
        .count()
}

#[allow(unused)]
fn are_safe_deltas_lossy_slow(deltas: Vec<i64>) -> bool {
    let negative = deltas.iter().map(Neg::neg).collect_vec();

    [deltas, negative].into_iter().any(|deltas| {
        if are_safe_deltas(&deltas)
            || are_safe_deltas(&deltas[1..])
            || are_safe_deltas(&deltas[..=deltas.len() - 2])
        {
            return true;
        }
        for i in 1..deltas.len() {
            if are_safe_deltas(&deltas[..i - 1])
                && is_safe_delta(&(deltas[i - 1] + deltas[i]))
                && are_safe_deltas(&deltas[i + 1..])
            {
                return true;
            }
        }
        false
    })
}

fn are_safe_deltas_lossy_fast(deltas: Vec<i64>) -> bool {
    let negative = deltas.iter().map(Neg::neg).collect_vec();

    [deltas, negative].into_iter().any(|deltas| {
        for (i, delta) in deltas.iter().enumerate() {
            if !is_safe_delta(delta) {
                let combine_previous = if i == 0 {
                    are_safe_deltas(&deltas[i + 1..])
                } else {
                    is_safe_delta(&(deltas[i - 1] + delta)) && are_safe_deltas(&deltas[i + 1..])
                };
                let combine_next = if i == deltas.len() - 1 {
                    true
                } else {
                    is_safe_delta(&(deltas[i + 1] + delta)) && are_safe_deltas(&deltas[i + 2..])
                };
                return combine_previous || combine_next;
            }
        }
        true
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Input) -> usize {
    input
        .iter()
        .map(|line| compute_deltas(line).collect_vec())
        .map(are_safe_deltas_lossy_fast)
        .filter(|v| *v)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let i = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let i = input_generator(i);

        assert_eq!(solve_part2(&i), 4);
    }

    #[test]
    fn test_part2_b() {
        let vec = compute_deltas(&[22, 18, 20, 18, 17, 16]).collect_vec();
        dbg!(&vec);
        assert_eq!(
            are_safe_deltas_lossy_slow(vec.clone()),
            are_safe_deltas_lossy_fast(vec)
        );
    }
}
