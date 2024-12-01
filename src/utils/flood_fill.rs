use std::collections::{HashSet, VecDeque};

use super::{position::{IndexInBoundsExt, NeighborsExt}, ToIndex};
use cgmath::Vector2;
use ndarray::{ArrayBase, Data, Ix2};

pub fn flood_fill<S>(
    arr: &ArrayBase<S, Ix2>,
    start: Vector2<usize>,
    mut can_paint: impl FnMut(&S::Elem) -> bool,
    mut callback_paint: impl FnMut(Vector2<usize>),
) where
    S: Data,
{
    let mut visited = arr.map(|_| false);
    let mut queue = VecDeque::new();

    if can_paint(arr.get(start.to_index()).unwrap()) {
        queue.push_back(start);
        visited[start.to_index()] = true;
    }

    while let Some(pos) = queue.pop_front() {
        callback_paint(pos);

        for neigh in pos.get_neighbors_ortho() {
            if arr.is_index_in_bounds(neigh.to_index())
                && !visited[neigh.to_index()]
                && can_paint(&arr[neigh.to_index()])
            {
                queue.push_back(neigh);
                visited[neigh.to_index()] = true;
            }
        }
    }
}

pub fn flood_fill_set_impl<S>(
    arr: &ArrayBase<S, Ix2>,
    start: Vector2<usize>,
    mut can_paint: impl FnMut(&S::Elem) -> bool,
    mut callback_paint: impl FnMut(Vector2<usize>),
) where
    S: Data,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    if can_paint(arr.get((start.y, start.x)).unwrap()) {
        queue.push_back(start);
    }

    while let Some(pos) = queue.pop_front() {
        callback_paint(pos);
        visited.insert(pos);

        for neigh in pos.get_neighbors_ortho() {
            if arr.is_index_in_bounds([neigh.y, neigh.x])
                && !visited.contains(&neigh)
                && can_paint(&arr[(neigh.y, neigh.x)])
            {
                queue.push_back(neigh);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use cgmath::vec2;
    use chumsky::{error::Cheap, prelude::*};
    use ndarray::Array2;

    use crate::utils::array_from_vector;

    use super::flood_fill;

    fn parse_array(s: &str) -> Array2<bool> {
        just::<_, _, Cheap<_>>('#')
            .to(true)
            .or(just(' ').to(false))
            .repeated()
            .separated_by(text::newline())
            .map(array_from_vector)
            .parse(s)
            .unwrap()
    }

    #[test]
    fn test_simple_fill() {
        let mut set = HashSet::new();
        let array = parse_array(
            r#"###
# #
###"#,
        );

        flood_fill(
            &array,
            vec2(1, 1),
            |&v| !v,
            |vec| {
                set.insert(vec);
            },
        );

        assert_eq!(set, HashSet::from_iter([vec2(1, 1)]));
    }

    #[test]
    fn test_simple_fill2() {
        let mut set = HashSet::new();
        let array = parse_array(
            r#"###
   
#  "#,
        );

        flood_fill(
            &array,
            vec2(0, 1),
            |&v| !v,
            |vec| {
                set.insert(vec);
            },
        );

        assert_eq!(
            set,
            HashSet::from_iter([vec2(0, 1), vec2(1, 1), vec2(2, 1), vec2(1, 2), vec2(2, 2)])
        );
    }

    #[test]
    fn test_simple_fill3() {
        let mut set = HashSet::new();
        let array = parse_array(
            r#"   
###
   "#,
        );

        flood_fill(
            &array,
            vec2(0, 0),
            |&v| !v,
            |vec| {
                set.insert(vec);
            },
        );

        assert_eq!(
            set,
            HashSet::from_iter([vec2(0, 0), vec2(1, 0), vec2(2, 0)])
        );
    }
}
