use std::{fmt::Debug, str::FromStr};

use chumsky::{prelude::*, text::Character, Error};
use ndarray::Array2;
use num_traits::Num;

use super::array_from_vector;

pub trait GridParser<T, E: Error<char>>: Parser<char, T, Error = E> + Sized {
    fn grid(self) -> impl Parser<char, Array2<T>, Error = E> {
        self.repeated()
            .at_least(1)
            .separated_by(text::newline())
            .at_least(1)
            .map(array_from_vector)
    }
}

impl<T, E: Error<char>, P: Parser<char, T, Error = E>> GridParser<T, E> for P {}

pub fn number<C: Clone + Character, E: Error<C>, T: Num + FromStr>() -> impl Parser<C, T, Error = E>
where
    <C as Character>::Collection: AsRef<str>,
    <T as FromStr>::Err: Debug,
{
    text::int::<C, E>(10).from_str().unwrapped()
}

pub fn number_signed<E: Error<char>, T: Num + FromStr>() -> impl Parser<char, T, Error = E>
where
    <T as FromStr>::Err: Debug,
{
    just('-')
        .or_not()
        .chain::<char, _, _>(text::int::<char, _>(10))
        .collect::<String>()
        .from_str::<T>()
        .unwrapped()
}

pub fn digit<E: Error<char>>() -> impl Parser<char, u8, Error = E> {
    filter(|c: &char| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as u8)
}

pub fn alphanumeric<E: Error<char>>() -> impl Parser<char, char, Error = E> {
    filter(|c: &char| c.is_alphanumeric())
}
