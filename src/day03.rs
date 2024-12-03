use chumsky::prelude::*;

use crate::utils::parsers::digit;

pub enum Instruction {
    Mul(u16, u16),
    Do,
    Dont,
}

type Input = Vec<Instruction>;

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    let num = || {
        digit().repeated().at_least(1).at_most(3).map(|digits| {
            digits
                .into_iter()
                .fold(0u16, |acum, v| acum * 10 + v as u16)
        })
    };

    let mul = just("mul(")
        .ignore_then(num())
        .then_ignore(just(','))
        .then(num())
        .then_ignore(just(')'));

    let do_inst = just("do()").ignored();
    let dont_inst = just("don't()").ignored();

    mul.map(|(l, r)| Instruction::Mul(l, r))
        .or(do_inst.map(|_| Instruction::Do))
        .or(dont_inst.map(|_| Instruction::Dont))
        .map(Some)
        .or(chumsky::primitive::any().map(|_| None))
        .repeated()
        .at_least(1)
        .flatten()
        .then_ignore(end())
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    parser().parse(input).unwrap()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    input
        .iter()
        .filter_map(|inst| match inst {
            Instruction::Mul(l, r) => Some((l, r)),
            _ => None,
        })
        .map(|(l, r)| *l as u64 * *r as u64)
        .sum::<u64>()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let mut enabled = true;
    let mut acum = 0;
    for inst in input {
        match *inst {
            Instruction::Mul(l, r) if enabled => {
                acum += l as u64 * r as u64;
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
            _ => {}
        }
    }
    acum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let i = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let i = input_generator(i);

        assert_eq!(solve_part2(&i), 48);
    }
}
