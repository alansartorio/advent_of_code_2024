use itertools::Itertools;
use num::Integer;
use std::{
    collections::{btree_map, BTreeMap},
    iter::{self, Map},
    ops::{Add, Bound, RangeInclusive},
};

pub trait MyRange<T> {
    fn contains(&self, other: &RangeInclusive<T>) -> bool;
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool;
    fn cut(&self, pos: T) -> [Option<RangeInclusive<T>>; 2];
}
impl<T: PartialOrd + Integer + Add<T> + Copy> MyRange<T> for RangeInclusive<T> {
    fn contains(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.end() >= self.start() && other.start() <= self.end()
    }

    fn cut(&self, pos: T) -> [Option<RangeInclusive<T>>; 2] {
        let allowed_cutting_positions = *self.start() + T::one()..=*self.end();
        let mut res = [None, None];
        if RangeInclusive::contains(&allowed_cutting_positions, &pos) {
            res[0] = Some(*self.start()..=pos - T::one());
            res[1] = Some(pos..=*self.end());
        } else if pos < *allowed_cutting_positions.start() {
            res[1] = Some(self.clone());
        } else {
            res[0] = Some(self.clone());
        }
        res
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ranges {
    ranges: BTreeMap<i64, i64>,
}

impl FromIterator<RangeInclusive<i64>> for Ranges {
    fn from_iter<T: IntoIterator<Item = RangeInclusive<i64>>>(iter: T) -> Self {
        let mut ranges = Ranges::default();

        for range in iter {
            ranges.add_range(range);
        }

        ranges
    }
}

impl Ranges {
    pub fn add_range(&mut self, range: RangeInclusive<i64>) {
        //println!("Adding {range:?}");
        let intersecting_ranges = self
            .ranges
            .extract_if(|start, end| (*start..=*end).overlaps(&range))
            .map(|(start, end)| start..=end)
            .collect_vec();

        let min = *intersecting_ranges
            .first()
            .map_or(range.start(), |first| first.start().min(range.start()));

        let max = *intersecting_ranges
            .last()
            .map_or(range.end(), |last| last.end().max(range.end()));

        self.ranges.insert(min, max);
    }

    pub fn extract_range(&mut self, range: RangeInclusive<i64>) -> Ranges {
        let mut intersecting_ranges = self
            .ranges
            .extract_if(|start, end| (*start..=*end).overlaps(&range))
            .map(|(start, end)| start..=end)
            .collect_vec();

        let mut remove_first = false;
        if let Some(first) = intersecting_ranges.first_mut() {
            let [left, right] = first.cut(*range.start());
            if let Some(left) = left {
                self.ranges.insert(*left.start(), *left.end());
            }
            if let Some(right) = right {
                *first = right;
            } else {
                remove_first = true;
            }
        }

        if remove_first {
            intersecting_ranges.remove(0);
        }

        let mut remove_last = false;
        if let Some(last) = intersecting_ranges.last_mut() {
            let [left, right] = last.cut(*range.end() + 1);
            if let Some(left) = left {
                *last = left;
            } else {
                remove_last = true;
            }
            if let Some(right) = right {
                self.ranges.insert(*right.start(), *right.end());
            }
        }

        if remove_last {
            intersecting_ranges.remove(intersecting_ranges.len() - 1);
        }

        intersecting_ranges.into_iter().collect()
    }

    pub fn substract_from(&self, range: RangeInclusive<i64>) -> Self {
        fn intersect_ranges(
            r1: RangeInclusive<i64>,
            r2: RangeInclusive<i64>,
        ) -> Option<RangeInclusive<i64>> {
            if r1.end() >= r2.start() && r1.start() <= r2.end() {
                let max_start = *r1.start().max(r2.start());
                let min_end = *r1.end().min(r2.end());
                Some(max_start..=min_end)
            } else {
                None
            }
        }

        let bounds = self
            .ranges
            .iter()
            .map(|(&start, &end)| (Bound::Included(start), Bound::Included(end)));

        let unbounded = [(Bound::<i64>::Unbounded, Bound::<i64>::Unbounded)];

        unbounded
            .into_iter()
            .chain(bounds)
            .chain(unbounded.into_iter())
            .tuple_windows()
            .map(|(r1, r2)| (r1.1, r2.0))
            .map(|(start, end)| {
                RangeInclusive::new(
                    match start {
                        Bound::Unbounded => *range.start(),
                        Bound::Included(s) => (*range.start()).max(s + 1),
                        _ => unreachable!(),
                    },
                    match end {
                        Bound::Unbounded => *range.end(),
                        Bound::Included(e) => (*range.end()).min(e - 1),
                        _ => unreachable!(),
                    },
                )
            })
            .filter(|r| !r.is_empty())
            .filter_map(|r| intersect_ranges(r, range.clone()))
            .collect()
    }

    pub fn remove_pos(&mut self, pos: i64) {
        if let Some((start, end)) = self
            .ranges
            .extract_if(|start, end| *end >= pos && *start <= pos)
            .next()
        {
            if start != pos {
                self.ranges.insert(start, pos - 1);
            }
            if end != pos {
                self.ranges.insert(pos + 1, end);
            }
        }
    }

    pub fn cut_range(&mut self, pos: i64) {
        if let Some((start, end)) = self
            .ranges
            .extract_if(|start, end| *end >= pos && *start <= pos)
            .next()
        {
            if start != pos {
                self.ranges.insert(start, pos - 1);
            }
            if end != pos {
                self.ranges.insert(pos, end);
            }
        }
    }

    pub fn get_positions(
        &self,
    ) -> iter::FlatMap<
        btree_map::Iter<i64, i64>,
        <RangeInclusive<i64> as IntoIterator>::IntoIter,
        fn((&i64, &i64)) -> RangeInclusive<i64>,
    > {
        self.ranges.iter().flat_map(|(&start, &end)| start..=end)
    }

    pub fn count(&self) -> u64 {
        let mut count = 0;
        for (start, end) in &self.ranges {
            count += end.abs_diff(*start) + 1;
        }

        count
    }

    pub fn iter_ranges(&self) -> btree_map::Iter<i64, i64> {
        self.ranges.iter()
    }
}

impl IntoIterator for Ranges {
    type Item = RangeInclusive<i64>;
    type IntoIter =
        Map<<BTreeMap<i64, i64> as IntoIterator>::IntoIter, fn((i64, i64)) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter().map(|(start, end)| start..=end)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::Ranges;

    #[test]
    fn test_ranges() {
        let mut ranges = Ranges::default();

        ranges.add_range(1..=10);
        ranges.add_range(5..=15);
        ranges.add_range(30..=35);
        ranges.add_range(25..=26);

        assert_eq!(ranges.count(), 23);

        ranges.add_range(1..=50);

        assert_eq!(ranges.count(), 50);
    }

    #[test]
    fn test_range_subs() {
        let ranges = Ranges::from_iter([0..=5, 10..=15]);
        let ranges = ranges.substract_from(0..=20);

        assert_eq!(
            ranges.get_positions().collect_vec(),
            vec![6, 7, 8, 9, 16, 17, 18, 19, 20]
        )
    }

    #[test]
    fn test_range_subs_2() {
        let ranges = Ranges::from_iter([0..=5, 10..=15]);
        let ranges = ranges.substract_from(3..=15);

        assert_eq!(ranges.get_positions().collect_vec(), vec![6, 7, 8, 9])
    }

    #[test]
    fn test_range_subs_3() {
        let ranges = Ranges::from_iter([0..=5, 10..=14]);
        let ranges = ranges.substract_from(3..=15);

        assert_eq!(ranges.get_positions().collect_vec(), vec![6, 7, 8, 9, 15])
    }

    #[test]
    fn test_range_subs_4() {
        let ranges = Ranges::from_iter([0..=5, 10..=11]);
        let ranges = ranges.substract_from(7..=10);

        assert_eq!(ranges.get_positions().collect_vec(), vec![7, 8, 9])
    }

    #[test]
    fn test_range_subs_inside() {
        let ranges = Ranges::from_iter([0..=5, 10..=11]);
        let ranges = ranges.substract_from(7..=8);

        assert_eq!(ranges.get_positions().collect_vec(), vec![7, 8])
    }

    #[test]
    fn test_range_subs_inside_2() {
        let ranges = Ranges::from_iter([0..=5, 10..=11]);
        let ranges = ranges.substract_from(1..=4);

        assert_eq!(ranges.get_positions().collect_vec(), vec![])
    }

    #[test]
    fn test_extract_range() {
        let mut ranges = Ranges::from_iter([0..=5, 10..=11]);

        let intersect = ranges.extract_range(3..=10);

        assert_eq!(intersect.get_positions().collect_vec(), vec![3, 4, 5, 10]);
        assert_eq!(ranges.get_positions().collect_vec(), vec![0, 1, 2, 11]);
    }

    #[test]
    fn test_extract_range_2() {
        let mut ranges = Ranges::from_iter([0..=1, 3..=4, 6..=7, 9..=10]);

        let intersect = ranges.extract_range(4..=6);

        assert_eq!(intersect.get_positions().collect_vec(), vec![4, 6]);
        assert_eq!(
            ranges.get_positions().collect_vec(),
            vec![0, 1, 3, 7, 9, 10]
        );
    }
}
