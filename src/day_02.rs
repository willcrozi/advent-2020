use std::ops::RangeInclusive;
use std::str::FromStr;

use regex::Regex;

static PASSWORD_DATA: &'static str = include_str!("../data/data_02.txt");

pub fn part_1() -> usize {
    parse_entries().into_iter()
        .filter(PwdEntry::is_valid_01)
        .count()
}

pub fn part_2() -> usize {
    parse_entries().into_iter()
        .filter(PwdEntry::is_valid_02)
        .count()
}

fn parse_entries() -> Vec<PwdEntry> {
    let re = Regex::new(r"^(\d+)-(\d+)\s+([a-zA-Z]):\s+([a-zA-Z]+)$").unwrap();

    PASSWORD_DATA.lines()
        .map(|s| {
            re.captures(s).map(|cap| {
                let letter = cap[3].chars().next().unwrap();
                let (low, high) = (usize::from_str(&cap[1]).unwrap(), usize::from_str(&cap[2]).unwrap());
                assert!(low <= high);
                let freq = low..=high;
                let pwd = cap[4].to_string();

                PwdEntry { letter, freq, pwd }
            }).unwrap()
        }).collect()
}

#[derive(Debug)]
struct PwdEntry {
    letter: char,
    freq: RangeInclusive<usize>,
    pwd: String,
}

impl PwdEntry {
    // Part 1.
    fn is_valid_01(&self) -> bool {
        let count = self.pwd.chars().filter(|c| *c == self.letter).count();
        self.freq.contains(&count)
    }

    // Part 2.
    fn is_valid_02(&self) -> bool {
        let vec = self.pwd.chars().collect::<Vec<_>>();
        let mut count = 0;

        if vec[*self.freq.start() - 1] == self.letter { count += 1; }
        if vec[*self.freq.end() - 1] == self.letter { count += 1; }

        count == 1
    }
}

