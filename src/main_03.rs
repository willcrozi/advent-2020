static WORLD_DATA: &'static str = include_str!("../data/data_03.txt");

fn main() {
    let world = World::init(WORLD_DATA);

    // Part 1.
    let mut tobog = Toboggan { world: &world, loc: (0, 0), trees: 0 };
    loop {
        if !tobog.travel(3, 1) { break; }
    }

    println!("Part 1: We hit {} trees!", tobog.trees);

    // Part 2.
    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let tree_counts = slopes.iter().map(|(x, y)| {
        let mut tobog = Toboggan { world: &world, loc: (0, 0), trees: 0 };
        loop {
            if !tobog.travel(*x, *y) { break; }
        }
        // println!("(Part 2 debug) Tree count: {}", tobog.trees);
        tobog.trees
    });

    println!("Part 2: The product of tree counts is: {}", tree_counts.fold(1, |acc, count| acc * count));
}

struct World {
    rows: Vec<Vec<Square>>
}

struct Toboggan<'a> {
    world: &'a World,
    loc: (usize, usize),
    trees: usize,
}

impl Toboggan<'_> {
    fn travel(&mut self, x: usize, y: usize) -> bool {
        let (mut loc_x, mut loc_y) = self.loc;

        loc_x += x;
        loc_y += y;

        self.loc = (loc_x, loc_y);

        // Return false if we have gone off the bottom of the map.
        if loc_y >= self.world.height() { return false; }

        let row = &self.world.rows[loc_y];
        if row[loc_x % self.world.width()] == Square::Tree { self.trees += 1; }

        true
    }
}

impl World {
    fn width(&self) -> usize { self.rows[0].len() }

    fn height(&self) -> usize { self.rows.len() }

    fn init(map: &str) -> World {
        let mut rows = vec![];

        let row_len = None;
        for row in map.lines() {
            let row = row.chars()
                .map(|c| {
                    match c {
                        '.' => Square::Empty,
                        '#' => Square::Tree,
                        _ => panic!("Invalid character in map input")
                    }
                }).collect::<Vec<Square>>();

            if let Some(len) = row_len {
                if row.len() != len { panic!("Row lengths are not consistent")}
            }

            rows.push(row);
        }

        World { rows }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Empty,
    Tree
}
