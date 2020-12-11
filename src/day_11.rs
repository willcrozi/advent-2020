use std::{cmp, mem};

static SEATS: &'static str = include_str!("../data/data_11.txt");

pub fn part_1() -> usize {
    let plan = seat_plan().collect::<Vec<_>>();
    count_stabilised(plan, 3, count_occupied_part_1)
}

pub fn part_2() -> usize {
    let plan = seat_plan().collect::<Vec<_>>();
    count_stabilised(plan, 4, count_occupied_part_2)
}

/// Iterates the seating plan layout according to initial state `plan`, the number of occupied seats
/// tolerated `tol`, and the method of determining relevant occupied seats `count_occ_op`.
fn count_stabilised<F>(mut plan: Vec<Vec<Loc>>, tol: usize, count_occ_op: F) -> usize
    where F: Fn(&[Vec<Loc>], (usize, usize)) -> usize
{
    let mut plan1 = plan.clone();

    let (mut cur, mut next) = (&mut plan[..], &mut plan1[..]);
    let mut modded = true;

    while modded {
        modded = false;

        for y in 0..cur.len() {
            for x in 0..cur[y].len() {
                let occupied = count_occ_op(cur, (x, y));

                next[y][x] = gen_next(&cur[y][x], occupied, tol);
                modded |= next[y][x] != cur[y][x];
            }
        }
        mem::swap(&mut cur, &mut next);
    }

    cur.iter().flatten().filter(|l| **l == Loc::Occupied).count()

}

/// Returns the number of occupied seats adjacent to the location (x, y)
fn count_occupied_part_1(plan: &[Vec<Loc>], (x, y): (usize, usize)) -> usize {
    let count =
        // Iterate over the adjacent rows.
        plan[y.saturating_sub(1)..cmp::min(y + 2, plan.len())]
        .iter()
        // Iterate over adjacent columns within.
        .flat_map(|row| &row[x.saturating_sub(1)..cmp::min(x + 2, row.len())])
        // Count the occupied seats.
        .filter(|l| **l == Loc::Occupied)
        .count();

    // Remove current seat from count if occupied.
    if plan[y][x] == Loc::Occupied { count - 1 } else { count }
}

/// Returns the number of occupied seats 'visible' to the location `(x, y)`
fn count_occupied_part_2(plan: &[Vec<Loc>], (x, y): (usize, usize)) -> usize {
    // Iterate over all 8 possible direction vectors.
    return [(-1_i64, 0_i64), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]
        .iter()
        // Sum the directions in which an occupied seat is visible.
        .map(|dir| count_visible_occ(plan, (x, y), dir))
        .sum();

    fn count_visible_occ(plan: &[Vec<Loc>], (x, y): (usize, usize), (d_x, d_y): &(i64, i64)) -> usize {
        let (mut x, mut y) = (x as i64, y as i64);
        loop {
            x += d_x;
            y += d_y;

            // Check for seating area boundary.
            if 0 > y || y >= plan.len() as i64 || 0 > x || x >= plan[y as usize].len() as i64 {
                return 0;
            }

            match plan[y as usize][x as usize] {
                Loc::Empty => return 0,
                Loc::Occupied => return 1,
                Loc::Floor => (),
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Loc {
    Floor,
    Empty,
    Occupied,
}

/// Determines next state of a location based on its current state `cur`, and `occ` the number of
/// occupied seats relevant to it.
fn gen_next(cur: &Loc, occ: usize, tol: usize) -> Loc {
    match (cur, occ) {
        // Subtract current location from `occ` if we are occupied.
        (Loc::Occupied, occ) if (occ > tol) => Loc::Empty,
        (Loc::Occupied, _) => Loc::Occupied,
        (Loc::Empty, 0) => Loc::Occupied,
        (Loc::Empty, _) => Loc::Empty,
        (Loc::Floor, _) => Loc::Floor,
    }
}

fn seat_plan() -> impl Iterator<Item=Vec<Loc>> {
    SEATS.lines()
        .map(|row| row.chars()
            .map(|loc| match loc {
                '.' => Loc::Floor,
                'L' => Loc::Empty,
                '#' => Loc::Occupied,
                _ => panic!("Invalid seat plan item")
            }).collect()
        )
}
