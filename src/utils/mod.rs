#![allow(dead_code)]

use itertools::Itertools;
use ndarray::Array2;

mod flood_fill;

mod position;
mod ranges;
pub mod y_combinator;
pub mod cycle_finder;
pub mod save_image;
pub mod grid_utils;
pub mod parsers;
pub mod graph_export;

pub use flood_fill::*;
pub use position::*;
pub use ranges::*;

pub trait FromRows<A> {
    fn from_rows<R: Iterator<Item = I>, I: Iterator<Item = A>>(rows: R) -> Self;
}
impl<A: Clone> FromRows<A> for Array2<A> {
    fn from_rows<R: Iterator<Item = I>, I: Iterator<Item = A>>(rows: R) -> Self {
        let data2 = rows.map(|r| r.collect_vec()).collect_vec();

        let width = data2.first().map(|r| r.len()).unwrap_or(0);
        let height = data2.len();

        Array2::from_shape_fn((height, width), |(y, x)| data2[y][x].clone())
    }
}

pub fn array_from_vector<T>(data: Vec<Vec<T>>) -> Array2<T> {
    let ncols = data.first().map_or(0, |row| row.len());
    let nrows = data.len();

    Array2::from_shape_vec((nrows, ncols), data.into_iter().flatten().collect_vec()).unwrap()
}
