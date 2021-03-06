use advent_2020::StrExt;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::collections::hash_map::RandomState;

static DECLARATIONS: &'static str = include_str!("../data/data_06.txt");

pub fn part_1() -> usize {
    DECLARATIONS.paragraphs(true)
        .map(|group| {
            let set: HashSet<char, RandomState> = HashSet::from_iter(
                group.chars()
                    .filter(char::is_ascii_alphabetic));

            set
        })
        .map(|g| g.len())
        .sum()
}

pub fn part_2() -> usize {
    DECLARATIONS.paragraphs(true)
        .map(|group| {
            // Create a set of all questions answered yes for each person in the group.
            let mut sets = group.lines().map(|person| {
                let set: HashSet<char, RandomState> = HashSet::from_iter(
                    person.chars()
                        .filter(char::is_ascii_alphabetic));
                set
            });

            let first = sets.next().unwrap();
            sets.fold(first, |acc, p| &acc & &p).len()
        })
        .sum()
}
