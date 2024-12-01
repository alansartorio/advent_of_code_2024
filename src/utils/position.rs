use std::{
    array,
    iter::{Filter, Flatten, Map},
};

use cgmath::{
    num_traits::{NumCast, Signed},
    BaseNum, Vector2,
};
use ndarray::{ArrayBase, Ix2, RawData};

//pub type Position<T> = Vector2<T>;

pub trait NeighborsExt: Sized {
    type Iter;
    fn get_neighbors_ortho(&self) -> Self::Iter;
}

pub trait IsPositiveExt: Sized {
    fn is_positive(&self) -> bool;
}

type SignedIter<T> = array::IntoIter<Vector2<T>, 4>;
type UnsignedIter<T> = Flatten<
    Map<
        Filter<SignedIter<i128>, fn(&Vector2<i128>) -> bool>,
        fn(Vector2<i128>) -> Option<Vector2<T>>,
    >,
>;

macro_rules! signed_neigh {
    ($($t:ty),+) => {
        $(
            impl NeighborsExt for Vector2<$t>
            {
                type Iter = SignedIter<$t>;

                fn get_neighbors_ortho(&self) -> Self::Iter {
                    [
                        self + Self::unit_y(),
                        self - Self::unit_y(),
                        self + Self::unit_x(),
                        self - Self::unit_x(),
                    ]
                    .into_iter()
                }
            }
        )*
    };
}

impl<T: BaseNum + Signed> IsPositiveExt for Vector2<T> {
    fn is_positive(&self) -> bool {
        !self.x.is_negative() && !self.y.is_negative()
    }
}

fn cast<T: NumCast>(v: Vector2<i128>) -> Option<Vector2<T>> {
    v.cast()
}

macro_rules! unsigned_neigh {
    ($($t:ty),+) => {
        $(
            impl NeighborsExt for Vector2<$t>
            {
                type Iter = UnsignedIter<$t>;

                fn get_neighbors_ortho(&self) -> Self::Iter {
                    self.cast::<i128>().unwrap().get_neighbors_ortho()
                        .filter(IsPositiveExt::is_positive as fn(&Vector2<i128>) -> bool)
                        .map(cast::<$t> as fn(Vector2<i128>) -> Option<Vector2<$t>>)
                        .flatten()
                }
            }
        )*
    };
}

signed_neigh!(i8, i16, i32, i64, i128);
unsigned_neigh!(u8, u16, u32, u64, usize);

pub trait IndexInBoundsExt<Idx> {
    fn is_index_in_bounds(&self, idx: Idx) -> bool;
}

impl<S> IndexInBoundsExt<[usize; 2]> for ArrayBase<S, Ix2>
where
    S: RawData,
{
    fn is_index_in_bounds(&self, idx: [usize; 2]) -> bool {
        let (h, w) = self.dim();
        (0..w).contains(&idx[1]) && (0..h).contains(&idx[0])
    }
}

pub trait ToIndex {
    fn to_index(&self) -> [usize; 2];
}

impl ToIndex for Vector2<usize> {
    fn to_index(&self) -> [usize; 2] {
        [self.y, self.x]
    }
}

#[cfg(test)]
mod tests {
    use cgmath::vec2;
    use itertools::Itertools;

    use super::NeighborsExt;

    #[test]
    fn test_neighbors() {
        assert_eq!(
            vec2(10i8, 10).get_neighbors_ortho().collect_vec(),
            vec![vec2(10, 11), vec2(10, 9), vec2(11, 10), vec2(9, 10)]
        );

        assert_eq!(
            vec2(10u8, 10).get_neighbors_ortho().collect_vec(),
            vec![vec2(10, 11), vec2(10, 9), vec2(11, 10), vec2(9, 10)]
        );

        assert_eq!(
            vec2(0u8, 0).get_neighbors_ortho().collect_vec(),
            vec![vec2(0, 1), vec2(1, 0)]
        );
    }
}
