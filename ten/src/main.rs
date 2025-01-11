use advent::prelude::*;
use common::{Coord, Digit, GetCoord};

#[derive(Clone, Copy, Debug, EnumIter)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn solve(input: Grid<Digit>, use_peaks: bool) -> usize {
    let start_positions: Vec<Coord> = input
        .positions()
        .map(|(x, y)| Coord(x, y))
        .filter(|coord| input.get_coord(*coord).is_some_and(|d| d.0 == 0))
        .collect();

    start_positions
        .iter()
        .map(|start_pos| {
            let mut trails_found = 0;
            let mut peaks_found = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(*start_pos);

            while let Some(coord) = queue.pop_front() {
                let current_digit = input.get_coord(coord).unwrap();
                if current_digit.0 == 9 {
                    trails_found += 1;
                    peaks_found.insert(coord);
                    continue;
                }

                for dir in Dir::iter() {
                    let next_coord = match dir {
                        Dir::N => Coord(coord.0, coord.1 + 1),
                        Dir::E => Coord(coord.0 + 1, coord.1),
                        Dir::S => Coord(coord.0, coord.1.saturating_sub(1)),
                        Dir::W => Coord(coord.0.saturating_sub(1), coord.1),
                    };

                    input
                        .get_coord(next_coord)
                        .is_some_and(|next_digit| current_digit.0 + 1 == next_digit.0)
                        .then(|| queue.push_back(next_coord));
                }
            }

            if use_peaks {
                peaks_found.len()
            } else {
                trails_found
            }
        })
        .sum()
}

#[part_one]
fn part_one(input: Grid<Digit>) -> usize {
    solve(input, true)
}

#[part_two]
fn part_two(input: Grid<Digit>) -> usize {
    solve(input, false)
}

harness!(part_1: 796);
