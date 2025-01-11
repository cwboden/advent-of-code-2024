use advent::prelude::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coord(i32, i32);

#[derive(HasParser, PartialEq, Eq, Copy, Clone)]
enum Space {
    #[parse(string = ".")]
    Empty,
    #[parse(string = "#")]
    Block,
    Guard(GuardDir),
}

struct Guard {
    dir: GuardDir,
    coord: Coord,
    visited: HashSet<(Coord, GuardDir)>,
}

enum AdvanceResult {
    Moved(Coord),
    StuckInLoop,
    WalkedOut,
}

#[derive(Hash, HasParser, PartialEq, Eq, Copy, Clone)]
enum GuardDir {
    #[parse(string = "^")]
    N,
    #[parse(string = ">")]
    E,
    #[parse(string = "v")]
    S,
    #[parse(string = "<")]
    W,
}

impl GuardDir {
    fn advance(&self, coord: &Coord) -> Coord {
        match &self {
            Self::N => Coord(coord.0, coord.1 + 1),
            Self::E => Coord(coord.0 + 1, coord.1),
            Self::S => Coord(coord.0, coord.1 - 1),
            Self::W => Coord(coord.0 - 1, coord.1),
        }
    }

    fn turn(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

impl Guard {
    fn new(coord: Coord) -> Self {
        Self {
            dir: GuardDir::W,
            coord,
            visited: HashSet::new(),
        }
    }

    fn turn(&mut self) {
        self.dir = self.dir.turn()
    }

    fn advance(&mut self, map: &Grid<Space>) -> AdvanceResult {
        let ahead = self.dir.advance(&self.coord);

        if ahead.0 < 0
            || ahead.0 >= map.width().try_into().unwrap()
            || ahead.1 < 0
            || ahead.1 >= map.height().try_into().unwrap()
        {
            // Elfis has left the building
            return AdvanceResult::WalkedOut;
        }

        match map[ahead.0.try_into().unwrap()][ahead.1.try_into().unwrap()] {
            Space::Block => {
                // turn and advance immediately
                self.turn();
                self.advance(map)
            }
            Space::Empty => {
                if self.visited.contains(&(ahead.clone(), self.dir)) {
                    // we're going in circles
                    AdvanceResult::StuckInLoop
                } else {
                    self.visited.insert((ahead.clone(), self.dir));
                    self.coord = ahead.clone();
                    AdvanceResult::Moved(ahead)
                }
            }
            Space::Guard { .. } => {
                panic!("two guards!?")
            }
        }
    }

    fn get_path(&mut self, map: &Grid<Space>) -> HashSet<Coord> {
        let mut coords = HashSet::new();
        coords.insert(self.coord.clone());

        while let AdvanceResult::Moved(coord) = self.advance(&map) {
            coords.insert(coord);
        }
        coords
    }

    fn has_loop(&mut self, map: &Grid<Space>) -> bool {
        loop {
            match self.advance(&map) {
                AdvanceResult::Moved(_) => (),
                AdvanceResult::StuckInLoop => return true,
                AdvanceResult::WalkedOut => return false,
            }
        }
    }
}

// fn print_map(map: Grid<Space>, coords: &HashSet<Coord>) {
//     for (x, y) in map.positions() {
//         if y == 0 {
//             print!("\n");
//         }

//         match map[x][y] {
//             Space::Block => print!("#"),
//             Space::Guard(dir) => match dir {
//                 GuardDir::N => print!("^"),
//                 GuardDir::E => print!(">"),
//                 GuardDir::S => print!("v"),
//                 GuardDir::W => print!("<"),
//             },
//             Space::Empty => {
//                 if coords.contains(&Coord(x.try_into().unwrap(), y.try_into().unwrap())) {
//                     print!("X")
//                 } else {
//                     print!(".")
//                 }
//             }
//         }
//     }
//     print!("\n");
// }

fn get_start_coord(map: &mut Grid<Space>) -> Coord {
    let start_space = map
        .position(|cell| match cell {
            Space::Guard(_) => true,
            _ => false,
        })
        .unwrap();

    // clear out 'Guard' space
    map[start_space.0][start_space.1] = Space::Empty;

    Coord(
        start_space.0.try_into().unwrap(),
        start_space.1.try_into().unwrap(),
    )
}

#[part_one]
fn part_one(mut map: Grid<Space>) -> usize {
    let start_coord = get_start_coord(&mut map);
    let mut guard = Guard::new(start_coord.clone());

    let coords = guard.get_path(&map);
    // print_map(map, &coords);
    coords.len()
}

#[part_two]
fn part_two(mut map: Grid<Space>) -> usize {
    let start_coord = get_start_coord(&mut map);
    let mut guard = Guard::new(start_coord.clone());
    let coords = guard.get_path(&map);

    coords
        .iter()
        .skip(1 /* can't use starting coord */)
        .filter(|c| {
            let mut second_guard = Guard::new(start_coord.clone());

            let mut alternate_map = map.clone();
            alternate_map[c.0.try_into().unwrap()][c.1.try_into().unwrap()] = Space::Block;

            second_guard.has_loop(&alternate_map)
        })
        .count()
}

harness!(part_1: 4819, part_2: 1796);
