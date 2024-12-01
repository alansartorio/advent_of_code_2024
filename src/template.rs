use cgmath::vec2;
use chumsky::{prelude::*, text::newline};
use itertools::Itertools;
use ndarray::Array2;

use crate::utils::array_from_vector;

type Input = Array2<bool>;

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let pipe = just('#').to(true).or(just('.').to(false));

    pipe.repeated()
        .at_least(1)
        .separated_by(newline())
        .at_least(1)
        .map(array_from_vector)
        .then_ignore(end())
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    parser().parse(input).unwrap()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> usize {
    todo!("Implement solver");
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

