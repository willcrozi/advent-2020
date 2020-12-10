use std::cmp;
use std::cmp::Ordering;

static SEAT_DATA: &'static str = include_str!("../data/data_05.txt");

pub fn part_1() -> u32 {
    SEAT_DATA.lines()
        .map(parse_loc)
        .fold(0, |max, seat| cmp::max(seat.id(), max))
}

pub fn part_2() -> Option<u32> {
    let mut seats = SEAT_DATA.lines()
        .map(parse_loc)
        .collect::<Vec<_>>();

    seats.sort_unstable();
    let mut seats = seats.into_iter();
    let mut prev = seats.next().unwrap();

    let mut result = None;
    for cur in seats {
        assert!(cur.id() > prev.id());

        if cur.id() - prev.id() == 2 {
            result = Some(cur.id() - 1);
            break;
        }
        prev = cur;
    }

    result
}

#[derive(Debug, Eq)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        (self.row * 8) + self.col
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering { self.id().cmp(&other.id()) }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool { self.id().eq(&other.id()) }
}

/// Returns (row, col) of boarding pass entry, if valid.
fn parse_loc(loc: &str) -> Seat {
    let (row, col) = loc.split_at(7);
    debug_assert!(row.len() == 7);
    debug_assert!(col.len() == 3);

    let row = row.chars().enumerate()
        .fold(0, |row, (i, c)| match c.to_ascii_uppercase() {
            'B' => row + (64 >> i),
            'F' => row,
            _ => panic!(),
        });

    let col = col.chars().enumerate()
        .fold(0, |col, (i, c)| match c.to_ascii_uppercase() {
            'R' => col + (4 >> i),
            'L' => col,
            _ => panic!(),
        });

    Seat { row, col }
}
