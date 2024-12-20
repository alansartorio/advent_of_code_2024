use std::collections::{hash_set, HashSet};

use cgmath::{vec2, Vector2};
use itertools::{Itertools, MinMaxResult};
use ndarray::{Array2, ArrayBase, Axis, Data, Ix2};

use super::ToIndex;

pub fn print_map(map: &ArrayBase<impl Data<Elem = bool>, Ix2>) {
    let repr = map
        .rows()
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|v| if *v { "##" } else { "  " })
                .join("")
        })
        .join("\n");

    println!("{repr}");
}

pub trait GenerateBooleanMap {
    type Iter<'a>: Iterator<Item = &'a Vector2<i64>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;

    fn generate_boolean_map(&self) -> Array2<bool> {
        if !self.iter().any(|_| true) {
            return Array2::default((0, 0));
        }
        let (min_x, max_x) = match self.iter().map(|v| v.x).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(e) => (e, e),
            MinMaxResult::NoElements => unreachable!(),
        };
        let (min_y, max_y) = match self.iter().map(|v| v.y).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(e) => (e, e),
            MinMaxResult::NoElements => unreachable!(),
        };

        let w = max_x.abs_diff(min_x) as usize + 1;
        let h = max_y.abs_diff(min_y) as usize + 1;
        let min = vec2(min_x, min_y);

        let mut arr = Array2::from_elem((h, w), false);

        for &v in self.iter() {
            arr[(v - min).cast().unwrap().to_index()] = true;
        }

        arr
    }
}

impl GenerateBooleanMap for HashSet<Vector2<i64>> {
    type Iter<'a> = hash_set::Iter<'a, Vector2<i64>>;

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

pub fn move_elements<T>(arr: &mut Array2<T>, dir: Vector2<i8>)
where
    T: Clone + Copy + num_traits::Zero,
{
    for axis in [0, 1] {
        let roll = match axis {
            0 => dir.y,
            1 => dir.x,
            _ => unreachable!(),
        };
        let size = arr.shape()[axis];
        let cross_axis = 1 - axis;

        match roll.cmp(&0) {
            std::cmp::Ordering::Greater => {
                let dir_abs = roll as usize;
                for mut row in arr.axis_iter_mut(Axis(cross_axis)) {
                    for x in (dir_abs..size).rev() {
                        row[x] = row[x - dir_abs];
                    }
                    for x in 0..dir_abs {
                        row[x] = T::zero();
                    }
                }
            }
            std::cmp::Ordering::Less => {
                let dir_abs = (-roll) as usize;
                for mut row in arr.axis_iter_mut(Axis(cross_axis)) {
                    for x in 0..size - dir_abs {
                        row[x] = row[x + dir_abs];
                    }
                    for x in size - dir_abs..size {
                        row[x] = T::zero();
                    }
                }
            }
            std::cmp::Ordering::Equal => {}
        }
    }
}
