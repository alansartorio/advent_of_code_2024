#![feature(array_windows, type_ascription, map_try_insert, let_chains, if_let_guard, iter_array_chunks, btree_extract_if, hash_extract_if, extract_if, hash_raw_entry, ascii_char, array_chunks)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

mod utils;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

aoc_lib!{ year = 2024 }

