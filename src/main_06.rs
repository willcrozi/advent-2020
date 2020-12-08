use advent_2020::parse_paras;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::collections::hash_map::RandomState;

static DECLARATIONS: &'static str = include_str!("../data/data_06.txt");

fn main() {
    // Part 1.
    let groups = parse_paras(DECLARATIONS).map(|group| {
        let set: HashSet<char, RandomState> = HashSet::from_iter(
            group.chars()
                .filter(char::is_ascii_alphabetic));

        set
    });

    println!("Part 1: Sum of groups answers: {}", groups.map(|g| g.len()).sum::<usize>());

    // Part 2.
    let groups = parse_paras(DECLARATIONS).map(|para| {
        // Create a set of all questions answered yes for each person in the group.
        let mut sets = para.lines().map(|person| {
            let set: HashSet<char, RandomState> = HashSet::from_iter(
                person.chars()
                    .filter(char::is_ascii_alphabetic));
            set
        });

        let first = sets.next().unwrap();
        sets.fold(first, |acc, p| &acc & &p).len()
    });

    println!("Part 2: Sum of groups answers: {}", groups.sum::<usize>());
}
