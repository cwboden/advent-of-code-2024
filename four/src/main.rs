use advent::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    min: Coord,
    max: Coord,
}

#[derive(Clone, Copy, Debug, EnumIter)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    fn advance(&self, coord: &Coord, bounds: &Bounds) -> Option<Coord> {
        let dx = match self {
            Self::NE | Self::E | Self::SE => {
                if coord.x < bounds.max.x {
                    1
                } else {
                    return None;
                }
            }
            Self::SW | Self::W | Self::NW => {
                if coord.x > bounds.min.x {
                    -1
                } else {
                    return None;
                }
            }
            _ => 0,
        };
        let dy = match self {
            Self::N | Self::NE | Self::NW => {
                if coord.y < bounds.max.y {
                    1
                } else {
                    return None;
                }
            }
            Self::SE | Self::S | Self::SW => {
                if coord.y > bounds.min.y {
                    -1
                } else {
                    return None;
                }
            }
            _ => 0,
        };

        Some(Coord {
            x: coord.x.saturating_add_signed(dx),
            y: coord.y.saturating_add_signed(dy),
        })
    }

    fn advance_steps(&self, coord: Coord, bounds: &Bounds, steps: usize) -> Option<Coord> {
        (0..steps).fold(Some(coord), |current, _| match current {
            Some(pos) => self.advance(&pos, bounds),
            None => None,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Vector {
    coord: Coord,
    dir: Dir,
}

fn get_bounds(input: &Grid<char>) -> Bounds {
    Bounds {
        min: Coord { x: 0, y: 0 },
        max: Coord {
            x: input.width() - 1,
            y: input.height() - 1,
        },
    }
}

fn has_str(input: &Grid<char>, vector: &Vector, s: &str) -> bool {
    let bounds = get_bounds(input);
    s.chars()
        .fold(
            (true, Some(vector.coord)),
            |(current, pos), char| match pos {
                Some(coord) => (
                    current && input[coord.x][coord.y] == char,
                    vector.dir.advance(&coord, &bounds),
                ),
                None => (false, None),
            },
        )
        .0
}

#[part_one]
fn part_one(input: Grid<char>) -> usize {
    input
        .positions()
        .into_iter()
        .flat_map(|(x, y)| {
            Dir::iter().map(move |dir| Vector {
                coord: Coord { x, y },
                dir,
            })
        })
        .filter(|v| has_str(&input, v, "XMAS"))
        .count()
}

fn has_x_str(input: &Grid<char>, coord: Coord, s: &str) -> bool {
    let bounds = get_bounds(input);
    let length = s.len() - 1;
    let opposite_corner = Dir::SE.advance_steps(coord, &bounds, length);
    let s_corner = Dir::S.advance_steps(coord, &bounds, length);
    let e_corner = Dir::E.advance_steps(coord, &bounds, length);

    (has_str(
        input,
        &Vector {
            coord,
            dir: Dir::SE,
        },
        s,
    ) || opposite_corner.is_some_and(|other_coord| {
        has_str(
            input,
            &Vector {
                coord: other_coord,
                dir: Dir::NW,
            },
            s,
        )
    })) && (s_corner.is_some_and(|other_coord| {
        has_str(
            input,
            &Vector {
                coord: other_coord,
                dir: Dir::NE,
            },
            s,
        )
    }) || e_corner.is_some_and(|other_coord| {
        has_str(
            input,
            &Vector {
                coord: other_coord,
                dir: Dir::SW,
            },
            s,
        )
    }))
}

#[part_two]
fn part_two(input: Grid<char>) -> usize {
    input
        .positions()
        .into_iter()
        .filter(|(x, y)| has_x_str(&input, Coord { x: *x, y: *y }, "MAS"))
        .count()
}

harness!(part_1: 2547, part_2: 1939);
