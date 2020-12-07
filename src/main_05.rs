use std::cmp;
use std::cmp::Ordering;

fn main() {

    let mut max_seat_id = 0;

    let mut seats = BOARDING_PASSES.lines()
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

static BOARDING_PASSES: &'static str = "\
FFFBFFFRRL
FFFBBBFLLL
BFFFBFBRLR
BFBFFFBRLL
FFFBFBBRLR
FBBBBFBLRR
BFBBBFFLRR
FBFFBBFLRL
BFBBBBFRRL
FBBFBFFRRL
BBFFFBBLRR
BFFFFFBRLR
FBFFBFBLLR
FBBFFFFLRR
FFFFBBFRLR
BBBFBFBLRL
FFFBFFBRLL
BBBFBBBRLL
FBFFFBBRLR
BFBBBBBLRL
BFBFBBFRRR
BFFFBBBLRL
FBBFFBBRLR
FBFBFFFRLR
FBBFFBFLRR
BFFFFFFLLR
FBBFBBFRLR
BFBFBFFLLL
FFFBBBBRLL
FBFFFFBLLR
FFFBBBBLRR
FFFFBFBRRL
BBFBBBBLRL
BBBBFFBLRR
BFFBBFBRLR
BBBFFFFRLR
BBBBFFFRLR
FBBFFBBRRL
FFFFBFBRLL
FBFFFBFRLL
FBBBFBFLLR
FFFFBBFRLL
BBFFFBFLRL
FFBFBFBRLR
FBFFBBBLRR
FFFBFFBLRL
FFBFBBFLLR
FFBFFFBLLR
FBFBBBBRLL
FBBBBFBRLR
FBFFBBFRLL
FFBFFBFLLR
BBBFFBBLRL
FFFBFFFLLR
BBFBBBFRLR
BBFFFFBRRL
BFBFFFBLRL
BBFFBFBRRL
BBFBBFBRRR
FFBBBFBRRL
FFFBFBBRRL
BBFBBBFLRL
FBBFBBBRRR
FFBFFBFRLL
BBBFBFFLRL
FBFFFFFLLR
FFFBBFBLRL
BBFFBFBRLL
BBFBFFFRRL
FBFBBFFRLL
FBFBBFFLLR
BFBFFBFRLR
FBFBFFBLLL
BFFFBBBRLR
BBBFBFBRRL
BFFBFFBLRL
FBFFBFBLLL
FBBBFBFRLL
BFBFBFFLLR
BFFFFBFRLL
BFFBBFBLLR
FFBBBBFRRL
BFBBFBBRRR
FBFBFFFRRL
FFBFFFBRLR
BBBFBBFLRL
BFFBFFBLRR
FFBFBFBLRL
FFBFBFFLLR
BFBFBBBRRL
FFBFFFFLRR
BFFBBBBRLL
FFFBFFBRRR
BBBBFFBLRL
FFBFBFFLRL
BFFBFFFLRL
BBBFBFFRLR
BBBFBFFRLL
BBFBBBBRLL
FFFBBBFLLR
BFFFBBBRRR
BBFFFFFLLL
BBFBFBBLRR
BFBBFFBRRL
BFBBFFFRRR
BFBBBBFRLR
FBFFFFBRLR
FFBBFBFLRR
BFFBFFBRLL
FBFFFBBRRL
BBFFBBFRRR
FBFBFBFLRL
FBFBBFFRRL
BFFBFBFRLL
BBBFFBFLLR
FFBFBBBRRR
FBFFBBFLLL
FFBFFBBLRR
BBBFBBFRLL
FFBBFFFRRL
FFBFFBFLLL
FFFBBBBLRL
FFFBFBFRRR
BBFFFFFLRR
FBFBFBBLRR
BBFFFFBLRR
BBFFFBFLLR
FBFBFBFLLR
FFFFBBBLLL
FBBBFFFLRL
BFBBFFBLLR
FFBBFFFLRR
FBFBBFBLLR
FFFBBBFRRR
FBBFBFBRRR
FBBBBFFLRL
FFBFFBBRRL
BBFFFFFRRR
BFBBFFBLLL
FFBBFBFRRR
BBFFBBFRLL
BBBBFFBRRR
BBFBFBBRLR
BFBFFFFLRR
FFFBBBFRLR
BFBBBFBLRR
FBFBBBFLLR
FFBBFFFLRL
FBFBFFFLLL
BBBFFBBRRL
BBBBFFFLRL
BFBFFBBRLR
FBFBFBBLLL
BFFBBBBLLR
BBBFBBBRLR
BBFFFBBLLL
FBBBFFFLLR
BBFBBFFRLL
BFBBFFFLLL
FFFFBBBRLL
BFFBBBFRLL
BFBBBBFLRR
BFBBFBBRLL
BFFBBBFRRL
BFBFBBFRRL
FBBFFBFRRL
FFBFFBBLRL
FBBFFBBLRR
FBFBFBBLRL
FBFBBFBLRL
FBBBFFBRRL
BFFFFFBRRR
BBBFBBBLRL
FBBFFBFLLR
BFFBBBBRRR
FFFBFBBRRR
FFFBBFBLLR
FBBFFFFLRL
BBBFBFFLRR
BBBBFFBRLL
BFBBFBFLRR
FBFBFBBLLR
BFBFFBFRLL
BBBFFFBLLL
FBFFFBBLRL
BFBFFBBRRR
FBBFBFBRRL
FFFBFBFLRR
BBBFFBFLRL
BFBBFBBLRL
FBFFBBFRLR
BFFFFBFLRL
BFBBBFBLLR
BFBFBFFLRL
BFBBBBBLLL
FFFBBFBRLR
BBBBFBFLLL
BBFFFFFRLL
BBFBFBFRRL
FFFBBBBRRL
FFFBFBFRRL
BBBBFFBRRL
BFFBFFFLLL
BFBBBFBLRL
FFFBBFBRRL
FBFBBFFLRR
FFFBFFFRLL
BBFFBBBRRL
FBFFFFFLRR
BBBBFFFRRL
FFFBFFBLLR
BBFBBBFRRL
FFFBFBFLLL
FBBFFBFRLL
BFFFFBBRRL
BFBFBFBLRR
FBFBFBFLRR
BBFBBFBLLR
BBBFFBBLRR
FBBFFBFRLR
BBBFFFBLRL
FFBFFBFRLR
FFFBBFFLLR
FFBFFFBRLL
BFBFFFBRRL
FBBBFBFLRR
BBFFFBBLRL
BFFBBFBLRR
BFFFBBFLLR
BFBFBBBLLL
BFBFBBBRRR
FFBFBFFRLL
FFBBBBBRRL
FFBFFFFRRR
FFBBBBFRLR
FBFFFFBRRR
BFFBFFFLRR
BBFBFFBLRL
FBBBBBFRRR
BBFFBBFLLL
FFFBBBFLRL
FFBBFBBLLR
FFBFFFBRRL
FBBBFBBLRL
BFFBFBFLRL
FBBFFFFRLR
FFFBBFBRRR
BFFBFBBLRL
BFBBFBBLLR
FBBFBFFLLR
FFFFBFBRLR
BFFFFFBRLL
FBBBBBBLRL
BFFBBBFRLR
BBFBFBFRRR
BBFBFFFRLR
BBFBFFBLRR
FFFBFFFRLR
FBFBBBBRRL
BBBFFFBRLR
FFFBFFBLRR
FBFFFBFRRR
FFFFBBBRLR
BBBFFFBRLL
FFBBBFFLRR
BFBBFFBRRR
FBFBBBFLRL
BFBFFBFRRL
BBFBFBFRLR
BFBBBBBRLR
FFFFBBFLRR
FBFBBFBLRR
FFBFBFBLLL
FFFBBBBRRR
BBFFBBBRLR
BFFFBFBLLR
BFFFBFFLLR
FFFFBBFLRL
FBBBFBBRLL
BFBBFBBLRR
BBFBBFFRLR
BFBBFFFLLR
BBBFFFBLRR
BFFBBFFLRR
BBBFFBFRRL
BBFFBFFLLL
BFBFFFFRRR
FBFBBFFLRL
FBBBBBFLRL
FFBBFBFLRL
BFBBFBFLRL
FBBFFBBRLL
BBBFFFFLRL
BFFFBBBLLL
FFBBBFBRLR
FFBFBBFRRL
FBFFBBBRLL
BBFBBFBRLL
BFFFFFFLRR
BFFFFBBRRR
FFFFBBFLLR
BFFFBBFLLL
BBFFFFBRRR
BFFBBBBLLL
BFFBFFBLLR
FFFFBBFRRL
FFBFBFFRRR
BFBFFFFLLL
BBBFFFBRRR
FFBBBFBRRR
FBBBFFBRLR
BFFBBBFLLL
FBFBBFFRLR
FBFFBBFRRR
FBFBFBFRLL
FBBBFFFLLL
BFFBFFBRRL
FBFFBBFLRR
BFBBBBBLRR
FBBBBBFRLR
FFFFBBBLRL
BBBFBFBLLL
BBFFBBBLRL
BFFBBFFRLL
BBBFFFFLLL
BBBBFFFLLR
BBFBBFFLRR
BFBFBFBLRL
BBFFFBBRRR
BBBFFFFLLR
FBBBFFBRLL
BFBBFFFRRL
FFBBBFBLRR
BFFBBBFLLR
FFBFBBBRLL
FBBBBBBLRR
BFFBBBBRRL
BFBBBFBLLL
FBBFBFBLRR
BFFFBFBRRR
FBBFFFBRRL
FFFBFBBLLR
BBBFFFFRRL
BFBBFFBRLR
FBBBFBBLRR
FBBFFFBLLR
FBBFBBFRLL
BBFBBBFLLR
BBFFFBFRRL
FBBBBFFRLR
FFBFBFBRRL
FFFBFFFRRR
BFFBFBFLLR
BFBFFFFRLL
BFFFFFBLLL
BFBBFFFLRR
FFBFFFBRRR
BBFFBBBRLL
FBFFFFFRLL
FFFBFFBRLR
FBFFBFFRLL
FFFBFFFLRL
BBFBFFBLLL
BBFFFFFLLR
FBBBFFFRLR
BFBFFBFLRR
FFFBBFBLRR
FBFFFBFLLR
BFFFBBFLRL
BFBBBBFRLL
FFBBFFFRLL
BBFBFBFLLL
FBFFBFFRRL
FBFBFBBRRL
FBFFFFFLLL
BBBFFFFRRR
BFBBBFFLLL
FBBFFFBLLL
FFBBBFFRLL
BFBBFBFRRR
FFBBBBFLRL
BFBBFFBRLL
FFBBBFFRRL
FBFBBBBLRL
FFBFBFBLRR
FBFFBFBLRR
FFBFFFBLRL
BBFFBFBLLL
FBFBBBFRLL
FFBFFBFLRL
FBFBFBBRLR
BFFBBFBRRL
BFFFFFBLRL
FFBFBBBLRR
FBBBFFBLLL
BFBFBFFRRR
BBBFBFBLLR
FBFBBFFRRR
BFFFFFBRRL
FFBFFBBLLL
FBFFBBFRRL
FBBBBBBRLL
FFBBFBBRLL
BBFBFBBLLR
BBBFBFBLRR
BBFFBBBLLL
FBFFFFBLLL
BFBBFBFLLR
FBFFBFBRLL
BFFBFFFRRR
FFBBBFFRLR
FBFFBBFLLR
BBBFBFBRLL
FBBFFBFLLL
BFFBBBFLRL
BBBBFFFRRR
BBBFBBFRRR
FFBFBFBLLR
BFFBBFFRRL
BBFFFFFRLR
BBFBBBBRRR
BBFFBFBLRL
FBFFFFBLRR
FFBFBBFLRL
BFBFFBBLLL
BBBFFBFLLL
BBFBFFFLRR
BFBBBBBLLR
FFBFFFBLLL
FBFFFBBLLR
BFBFFBBLRR
FFBBBFFLRL
FBBFBFFLRR
FBBFBFFRLL
FBBFBFBRLR
FBBFFBBLLL
BBFFBFFLRL
FBFBBBFRRR
FFBFFBBRRR
FBBBBBFRLL
BBFFBFBLRR
BBBFFBBRRR
FFBBBBBRRR
BBFFBBBLLR
FBBFBFBLLL
BBBFBFFLLL
BBFFBBBLRR
BFBFBFFRLR
BBBFBBBLLL
FBBFBFBLLR
FBBBBFFLLR
BFFFBBFRLR
FFFFBFBRRR
BFBFBFBLLR
FFBFFBFRRL
BFBBFFBLRL
BFBBBFFLLR
BBFFBFFLRR
FBBBBFBRRR
FBFFFBBLRR
BBFBBBBLLL
FBFBBBBRLR
FFFBBFFLRR
FFBFFBBLLR
FBBBBBBRRR
FFFFBBFLLL
BFFBFFBRRR
FFBBFFBRLR
BBFFFFFLRL
BBBFBBBLRR
BFBFBFFLRR
BFFFBBFRLL
BFBBFBFLLL
BFBFFBFLLR
FFFFBBBLRR
BBFBFFBRLL
FFBBFBFRLR
BBBFBBBRRL
BFBFBFBRRR
BFFFFFFRLR
FBBBBFFLRR
FBFBFFBLRL
FFBBBFFLLR
FBBFFFFRLL
FBFBFBFRRL
BFFBFFFRLR
BFFFBFFRLL
BBFBFBBRLL
FFBBFBBLRR
FBFBBFFLLL
FBFBFFFLLR
BFFBBFFLLR
FFBBFFBRLL
FBBBFFBLRL
BBFBBBBLRR
FFBFBBFRRR
FFBFBBBLLR
FFBBFBFRRL
BBFFFBBRLR
FFBBFFBLLL
BFBFBBBLRR
FFBFBFBRLL
BFBBBFFRRL
BFBFBFBRLR
FFBFFBBRLL
FFBBBBFRLL
FBBBFBFRRL
BBFBFBFRLL
BFFBFFFRRL
BFFBBBFRRR
BBFBFFFLRL
FBBBFBBLLR
BFBFBBBRLR
BBBFBFBRRR
BFFBFBFRRL
FFBBFFFLLR
FFBFFFFRLR
FBFBFFBRLR
BFBFFFBRLR
FFBFFBFLRR
BBFBFBBLRL
BBFBFFFLLR
BFBFFBBLRL
FFBBFFBLRL
BFFBBBBLRR
BBFBBFFRRL
FBBFBFFLLL
BFBFBFFRRL
FFFBFFBLLL
FBBFFFBRLR
BFBBFBBRLR
BFFBBFFLRL
BFFFFFFLLL
FBFBBBFRLR
BFBFBFFRLL
BBFFBFFRRR
FBBBFBFLRL
FFFFBFBLLR
FBFBFBBRLL
FBBFFBFRRR
BBFBFFBLLR
FBFBBFBRRL
BFBBBFFLRL
BBBBFBFRLL
BFFFFFBLRR
BBBBFFFLLL
BFFBFBBLLR
FFFBBFBRLL
FBBBFBFLLL
BFFFBFFRLR
FBFFBFFRLR
BBFFFFBLRL
BFFBBFBRLL
FFFBBFFRRR
FFBBFBFLLL
BBBFFBFLRR
BFBBBFFRLL
BFBFFFFRRL
FFBBFBFLLR
BBFFBBFLRR
FFBBFBBRRR
FBBBBBBLLR
FFFBBBBLLR
BBFFFFBRLR
BFBFFBFLRL
FBFFFBBRLL
BBBFBBFLLR
BFFBBFBLLL
BFBBFBFRLL
BBFBFBFLLR
FFBBBFFLLL
FBBFBBBRLR
FBFFBFFLLL
BFBBBBBRRR
FFBFBBBRRL
BFFBFBBLLL
BBFFBFFLLR
FBFBFFBRRR
BBFBFFBRRL
BBBBFBFLLR
BFFFFFBLLR
FFFBFBBLRL
FBBFBBFLRL
BBBFFFBRRL
BFBBBFBRLL
FBFFFBFLRL
BBBFFBFRLL
BFBFFFFRLR
BFFFFFFLRL
BBFFFBBLLR
BBFBBFBLRR
BFBFBBBLRL
FFFFBBBRRL
FBBFBFBLRL
FBFBBBBLRR
FFBBBBFRRR
BBFFBFBRRR
BBFFFBFRRR
BFBBBFBRRR
BBFFBBFLLR
FBFFBBBLRL
BFBBFBFRRL
FBFBFBFRLR
BFBFFFBLLR
FBFFBFBRLR
BFBFFBBRLL
BFBFBFBRLL
FBBBFBFRRR
FBBBFBBRRL
FBFFFBFRLR
FBFBBBFRRL
BFBFFFBLRR
BBBBFBFLRL
BBBFBFFRRL
BFFFFFFRRL
BFBBBFBRRL
FBBBBFBLLL
BBFBBBFLLL
BFFFBBBRLL
FFBFBBFRLL
BBFBBFFLLL
BFFBFBFLLL
FFBBBBBLLL
FBBFFFFLLL
BBBFFFFLRR
BFFFFBFLRR
BFFBFBBLRR
BFFBFBBRLR
FBBBFFBLLR
BFFBFBFRLR
BBFBBBBRLR
BBFBBFBRLR
FFBFBBBRLR
BFFFBBFRRL
BBBFFBFRRR
BFBBFFFRLL
FFFBBFFRRL
FFFFBFBLLL
FFFFBFBLRL
BBBFFBBRLL
FFFBFBFRLR
BFFFBBFLRR
BFFFFBFLLL
FFBBFFBRRL
FBFFFBBRRR
FFBFBFFRRL
BFBBFFBLRR
BBFFBBFRRL
FFBBBBBLRR
BFBBFFFRLR
BFFFFBFLLR
FBFFBFFLLR
BFFFFBBRLL
BBFFBFBRLR
FBFBFFFLRL
FBFBFFFRLL
FBFFFFBRRL
BFFFBFBLRR
BFBFFBFRRR
FFBBFFFLLL
BFBFBBFLLR
FFFBFBBLRR
FBBBBBBLLL
BBBFBBBRRR
FBFFBFBRRR
FBBFBBBLLR
FBBFFBFLRL
BBFBBFBLLL
BBFBBBBRRL
FBFBBBBLLL
FBBBBFFRRR
FFBBFFBLRR
FBFFBBBRRR
FFBBFBBRLR
FFFBFFBRRL
BBBBFFBLLR
BBFBBFFLLR
BBFFBFFRLL
FBFFBBBRLR
FFBBBBFLRR
FBBFFBBLLR
FBBBBFBLLR
FBFBFFBLRR
BFFFBFFRRL
FBBBBFFRRL
FFBBFFBLLR
BBBFFBFRLR
BFBBBBFLLL
BBFBBBFRRR
FFFBFFFLRR
BBFFBBBRRR
FBBFBBFRRR
BFFBBFFRRR
FFBBFFFRRR
BFFBFBBRLL
FBBBFBBLLL
FBFBFFBRLL
FBBFBBBLLL
BFBFFFBLLL
FFFBFBFLLR
BBFFBFFRRL
BFBBBBBRRL
FBFFFFFLRL
BFFFBFFLLL
FBBFFFBRRR
FBBFBBFLRR
BFFFBBBRRL
BFBFFFFLRL
FBFFBFBLRL
BBBFFFBLLR
FBBFFBBLRL
FFFBFBFLRL
BBFFBFFRLR
FBFFFFBRLL
BBFBBFBLRL
BFFFBFFLRL
FBBFFFFLLR
BFFBBFFRLR
FFBFFFFLRL
FBFFFFBLRL
FBBBBBBRLR
BBFFFBBRLL
BFFFBBBLLR
BFFBBBBRLR
FBBBBFFLLL
FFBFBFFRLR
BFFFFBBLLL
FBFFFBFRRL
BFBFBBFRLL
FFBBBFBRLL
FBBFBBBLRR
FBBFFFBLRR
BFFFFFFRLL
FFBBBBBLLR
BBBBFBFLRR
BFBFBBBRLL
BFBBBBFLLR
FFFBFFFLLL
BFFBFBFRRR
BBBBFFFLRR
FBBFFFFRRR
BFFFBBFRRR
FBBFBFFRRR
BFBBFBBLLL
FFBBBBBRLR
BBBFFBBLLR
BBBFBFFRRR
BBFFFFBLLL
BFFBFFFLLR
FFBFBBFLRR
FFBBFFFRLR
FBFBBFBLLL
FBBBFBBRRR
FBFFBFBRRL
BFFFBFBLLL
FFFFBBBLLR
BFFBBFBLRL
BFFBBBBLRL
FBFFFFFRLR
FBFFBFFLRR
BFFFFBBLRR
FBBBFBFRLR
BBBFBBFLLL
FFBFFBBRLR
BBFBFBFLRR
FBFBBFBRLL
BFFBFBFLRR
FBBBBBFRRL
FBBBBBBRRL
BBFBFFFLLL
FFBBBFFRRR
BFBBBFBRLR
BFBFFFFLLR
BBBFFFFRLL
FBBFFFBRLL
BFFFFFFRRR
FBFFFFFRRR
FBFFBBBLLL
BFBFBBFLRR
BBFBBBBLLR
FBFBBBFLRR
FFFBBFFRLL
BFBFBBBLLR
FFBBFBBRRL
FBFBBBBLLR
BBFBFBBLLL
FBBFBBBRRL
FBBBBFFRLL
FBFBFBBRRR
BFBFBBFRLR
BFBBFBFRLR
FBFFBBBLLR
BFBFBFBRRL
BFFBFFFRLL
BFFFFBFRLR
FBBBFBBRLR
FFBFFFFRLL
FBBFBBFLLR
BFBFFBBRRL
FFBFBBBLRL
BBBFBFFLLR
FBFBFFFRRR
FFFBBBFRLL
BBBFBBBLLR
FBBBFFFRLL
FFBFBFFLLL
FFBBBBBLRL
BFBBBFFRLR
FFBBBBFLLL
FBFFFBFLLL
FFFBBBFLRR
BBFBBBFLRR
FBBFBBFRRL
FBBBBBFLRR
BBFFBBFLRL
FFBFFBFRRR
FBBBFFFRRL
FFBBBFBLRL
BBFFFBBRRL
FBBBFFFRRR
FFFFBBBRRR
FBBBBBFLLR
FBBBBFBRLL
FBFBBBBRRR
FFBFFFFLLR
BBBBFFBLLL
FBFFFFFRRL
BBFFFFBLLR
FFBBFBBLRL
FBFFFBFLRR
BFBFBBFLRL
BBBFBBFRRL
FFFBBFFLRL
FBBFBFFLRL
FBBFBFFRLR
BBFBFBBRRR
BFBFBBFLLL
BBFBBBFRLL
FBBFBBBLRL
BFFFFBFRRL
FBFBFBFRRR
FBFBBBFLLL
BFFFFBBLLR
BBFBBFFRRR
FFBFBFFLRR
BBFFFBFRLR
BBBFFBBRLR
FBFFBFFRRR
FFBFBBFLLL
BFFBBBFLRR
FBFBFBFLLL
BFFFFBBRLR
FFFBBFBLLL
BBBFBFBRLR
BFBFFBFLLL
FFFBBBBLLL
FFBFBFBRRR
FBFBBFBRRR
FBBFBBFLLL
BBFBFBFLRL
BFBBBBBRLL
FFBBFFBRRR
FFFFBBFRRR
BFBBBFFRRR
FFFBFBFRLL
FFBFFFFRRL
FFFBBBBRLR
FBBBFFBRRR
BBFFFBFLRR
FBFFFBBLLL
FBBBBBFLLL
BBFBFFFRLL
BFFFBBBLRR
FFBFBBBLLL
BBFFFBFRLL
FFBBBBBRLL
FBFFBFFLRL
FFFFBFBLRR
FBFBFFBRRL
BFFFFBFRRR
FBBBBFBLRL
BBBFBBFRLR
FFBBBBFLLR
FBFBBFBRLR
FBBFBBBRLL
FBBBFFBLRR
FBBFBFBRLL
FFBBFBFRLL
FFFBFBBRLL
BBFFBFBLLR
FBFBFFBLLR
BBFBFFBRLR
BFFBBFBRRR
BFFFBFBLRL
BFBFBFBLLL
BBBBFFFRLL
FBBFFBBRRR
BBFBFFBRRR
BBFBFBBRRL
BFBBBBFRRR
BFFFFBBLRL
BBBFFBBLLL
BFBBFFFLRL
BFFFBFFRRR
BFFBFFBRLR
FBBFFFBLRL
BFBBBBFLRL
FBFBFFFLRR
FFFBBBFRRL
BFFBFFBLLL
BFBBFBBRRL
BFFFBFBRRL
BFBFFBBLLR
FFBBBFBLLR
FFBBFBBLLL
FFFBFBBLLL
BBFBBFBRRL
BFFBBFFLLL
BFFFBFFLRR
BFBFFFBRRR
BBBBFFBRLR
FBFFBBBRRL
BBBFBBFLRR
BBFFFFBRLL
BBFBFFFRRR
FFBBBFBLLL
FBBBBFBRRL
BBFFFFFRRL
FFBFFFBLRR
BFFBFBBRRL
FFBFBBFRLR
FBBBFFFLRR
BBFFBBFRLR
FBBFFFFRRL
BBFBBFFLRL
FFBFFFFLLL
FFFBBFFLLL
BBFFFBFLLL
BFFFBFBRLL
FFFBBFFRLR
";