use advent_2020::StrExt;
use std::str::FromStr;

static SHIP_COMMANDS: &'static str = include_str!("../data/data_12.txt");

pub fn part_1() -> usize {
    let (loc, vec) = (Loc(0,0), Vector(1, 0));
    let (Loc(x, y), _) = commands(SHIP_COMMANDS)
        .fold((loc, vec), |(mut loc, mut vec), cmd| {
            match cmd {
                ("N", dist) => loc.1 -= dist,
                ("E", dist) => loc.0 += dist,
                ("S", dist) => loc.1 += dist,
                ("W", dist) => loc.0 -= dist,
                ("F", dist) => loc = loc.translate(vec.scale(dist)),
                ("L", deg) => vec = vec.rotate(-deg),
                ("R", deg) => vec = vec.rotate(deg),
                _ => panic!("Invalid ship command"),
            }
            (loc, vec)
        });

    (x.abs() + y.abs()) as usize
}

pub fn part_2() -> usize {
    let (loc, vec) = (Loc(0, 0), Vector(10, -1));
    let (Loc(x, y), _) = commands(SHIP_COMMANDS)
        .fold((loc, vec), |(mut loc, mut vec), cmd| {
                match cmd {
                    ("N", dist) => vec.1 -= dist,
                    ("E", dist) => vec.0 += dist,
                    ("S", dist) => vec.1 += dist,
                    ("W", dist) => vec.0 -= dist,
                    ("F", dist) => loc = loc.translate(vec.scale(dist)),
                    ("L", deg) => vec = vec.rotate(-deg),
                    ("R", deg) => vec = vec.rotate(deg),
                    _ => panic!("Invalid ship command"),
                }
                (loc, vec)
            }
        );

    (x.abs() + y.abs()) as usize
}

fn commands(cmd_src: &str) -> impl Iterator<Item=(&str, i32)>{
    cmd_src.lines()
        .map(|line| line.split_when(char::is_numeric).unwrap())
        .map(|(op, arg)| (op, i32::from_str(arg).unwrap()))
}

#[derive(Copy, Clone, Debug)]
struct Vector(i32, i32);

impl Vector {
    fn rotate(self, deg: i32) -> Self {
        let Vector (x, y) = self;
        match (4 + (deg / 90)) % 4  {
            0 => Vector(x, y),
            1 => Vector(-y, x),
            2 => Vector(-x, -y),
            3 => Vector(y, -x),
            _ => unreachable!()
        }
    }

    fn scale(self, scalar: i32) -> Vector{
        Vector(self.0 * scalar, self.1 * scalar)
    }
}

#[derive(Copy, Clone, Debug)]
struct Loc(i32, i32);

impl Loc {
    fn translate(self, vec: Vector) -> Loc { Loc(self.0 + vec.0, self.1 + vec.1) }
}
