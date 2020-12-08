use std::cmp;
use std::cmp::Ordering;

static SEAT_DATA: &'static str = include_str!("../data/data_05.txt");

fn main() {

    let mut max_seat_id = 0;

    let mut seats = SEAT_DATA.lines()
        .map(parse_loc)
        .map(|seat| {
            max_seat_id = cmp::max(seat.id(), max_seat_id);
            seat
        })
        .collect::<Vec<_>>();

    println!("Part 1: Max seat ID is {}", max_seat_id);

    // Part 2.
    seats.sort_unstable();
    let mut seats = seats.into_iter();
    let mut prev = seats.next().unwrap();
    // println!("{:?} id: {}", &prev, prev.id());

    for cur in seats {
        assert!(cur.id() > prev.id());
        // println!("{:?} id: {}", &cur, cur.id());

        if cur.id() - prev.id() == 2 {
            println!("Part 2: Your seat id is: {}", cur.id() - 1);
            break;
        }
        prev = cur;
    }
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
