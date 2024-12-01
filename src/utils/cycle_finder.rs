use std::{
    collections::{hash_map::Entry, HashMap},
    convert::identity,
    hash::Hash,
};

use itertools::{Either, Itertools};

pub struct CycleInfo {
    pub start: usize,
    pub length: usize,
}

pub struct StartAndCycle<T> {
    pub start: Vec<T>,
    pub cycle: Vec<T>,
}

pub trait CycleFindingGetAt<T: Clone + Hash + Eq>: Iterator<Item = T> {
    fn find_cycle_info(self, iter_by_state: &mut HashMap<T, usize>) -> Option<CycleInfo>
    where
        Self: Sized,
    {
        self.enumerate()
            .find_map(|(i, state)| match iter_by_state.entry(state) {
                Entry::Vacant(e) => {
                    e.insert(i);
                    None
                }
                Entry::Occupied(cycle_start) => {
                    let cycle_start = *cycle_start.get();
                    Some(CycleInfo {
                        start: cycle_start,
                        length: i - cycle_start,
                    })
                }
            })
    }

    fn find_cycle_map<FM: FnMut(T) -> M, M>(self, mut map: FM) -> StartAndCycle<M>
    where
        Self: Sized,
    {
        let mut cache = HashMap::<T, usize>::new();

        let cycle_info = self.find_cycle_info(&mut cache).unwrap_or(CycleInfo {
            start: cache.len(),
            length: 0,
        });

        let (start, cycle) =
            cache
                .into_iter()
                .sorted_by_key(|(_, i)| *i)
                .partition_map(|(state, i)| {
                    let state = map(state);
                    if i < cycle_info.start {
                        Either::Left(state)
                    } else {
                        Either::Right(state)
                    }
                });

        StartAndCycle { start, cycle }
    }

    fn find_cycle(self) -> StartAndCycle<T>
    where
        Self: Sized,
    {
        self.find_cycle_map(identity)
    }

    fn get_at(self, iterations: usize) -> T
    where
        Self: Sized,
    {
        let mut cache = HashMap::<T, usize>::new();

        let cycle_info = self.take(iterations + 1).find_cycle_info(&mut cache);

        let last_iteration = match cycle_info {
            Some(cycle_info) => {
                cycle_info.start + ((iterations - cycle_info.start) % cycle_info.length)
            }
            None => iterations,
        };

        cache
            .into_iter()
            .find_map(|(state, i)| (i == last_iteration).then_some(state))
            .unwrap()
    }
}

impl<T: Clone + Hash + Eq, I: Iterator<Item = T>> CycleFindingGetAt<T> for I {}
