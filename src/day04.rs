use cgmath::Vector2;
use chumsky::prelude::*;
use ndarray::Array2;
use text::newline;

use crate::utils::{array_from_vector, grid_utils::move_elements};

type Input = Array2<char>;

fn parser() -> impl Parser<char, Input, Error = Simple<char>> {
    one_of("XMAS")
        .repeated()
        .at_least(1)
        .separated_by(newline())
        .at_least(1)
        .map(array_from_vector)
        .then_ignore(end())
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    parser().parse(input).unwrap()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    let (height, width) = input.dim();
    let x = input.map(|c| *c == 'X');
    let m = input.map(|c| *c == 'M');
    let a = input.map(|c| *c == 'A');
    let s = input.map(|c| *c == 'S');

    [
        (1i8, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .into_iter()
    .map(Vector2::from)
    .map(|dir| {
        [&x, &m, &a, &s]
            .into_iter()
            .map(|mat| mat.map(|b| *b as u8))
            .enumerate()
            .map(|(i, mut letter)| {
                move_elements(&mut letter, dir * (i as i8));
                letter
            })
            .fold(Array2::<u8>::zeros((height, width)), |acum, elem| {
                acum + elem
            })
            .into_iter()
            .filter(|v| *v == 4)
            .count() as u64
    })
    .sum::<u64>()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let (height, width) = input.dim();
    let m = input.map(|c| *c == 'M');
    let a = input.map(|c| *c == 'A');
    let s = input.map(|c| *c == 'S');

    let find_for_directions = |dirs: [[i8; 2]; 2]| {
        dirs.into_iter()
            .map(Vector2::from)
            .map(|dir| {
                [&m, &a, &s]
                    .into_iter()
                    .map(|mat| mat.map(|b| *b as u8))
                    .enumerate()
                    .map(|(i, mut letter)| {
                        move_elements(&mut letter, dir * (i as i8 - 1));
                        letter
                    })
                    .fold(Array2::<u8>::zeros((height, width)), |acum, elem| {
                        acum + elem
                    })
                    .map(|v| *v == 3)
            })
            .fold(Array2::<u8>::zeros((height, width)), |acum, dir| {
                acum + dir.map(|v| *v as u8)
            })
    };

    (find_for_directions([[1, 1], [-1, -1]]) + find_for_directions([[1, -1], [-1, 1]]))
        .into_iter()
        .filter(|v| *v == 2)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let mat = array_from_vector(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        let mut other = mat.clone();
        move_elements(&mut other, Vector2::new(1, 0));
        assert_eq!(
            other,
            array_from_vector(vec![vec![0, 1, 2], vec![0, 4, 5], vec![0, 7, 8],])
        );

        move_elements(&mut other, Vector2::new(0, -1));
        assert_eq!(
            other,
            array_from_vector(vec![vec![0, 4, 5], vec![0, 7, 8], vec![0, 0, 0]])
        );
    }

    #[test]
    fn test_part1() {
        let i = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let i = input_generator(i);

        assert_eq!(solve_part1(&i), 18);
    }
}
