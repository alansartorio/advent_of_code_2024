use chumsky::prelude::*;
use itertools::Itertools;

use crate::utils::parsers::number;

type Input = Vec<(u32, u32)>;

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    number()
        .then_ignore(just("   "))
        .then(number())
        .separated_by(text::newline())
        .at_least(1)
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    parser().parse(input).unwrap()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let left = input.iter().map(|(a, _b)| a).sorted();
    let right = input.iter().map(|(_a, b)| b).sorted();
    left.zip(right).map(|(a, b)| a.abs_diff(*b)).sum::<u32>()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let left = input.iter().map(|(a, _b)| a).sorted();
    let right = input.iter().map(|(_a, b)| b).counts();
    left.map(|n| n * *right.get(n).unwrap_or(&0) as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let i = "";

        let i = input_generator(i);

        assert_eq!(solve_part1(&i), 374);
    }
}
