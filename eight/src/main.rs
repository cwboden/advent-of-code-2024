use advent::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Antenna(char);

impl HasParser for Antenna {
    #[into_parser]
    fn parser() -> _ {
        letter().or(digit()).map(Self)
    }
}

#[derive(HasParser)]
enum Input {
    Antenna(Antenna),
    #[parse(string = ".")]
    Empty,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

/// has parity -- get_antinode(one, two) != get_antinode(two, one)
fn get_antinode(one: Coord, two: Coord) -> Vec<Coord> {
    let dx = one.0.abs_diff(two.0);
    let dy = one.1.abs_diff(two.1);

    let coord_xs = if one.0 < two.0 {
        [one.0 - dx, two.0 + dx]
    } else {
        [one.0 + dx, two.0 - dx]
    };

    let coord_ys = if one.1 < two.1 {
        [one.1 - dy, two.1 + dy]
    } else {
        [one.1 + dy, two.1 - dy]
    };

    coord_xs
        .into_iter()
        .zip(coord_ys)
        .map(|(x, y)| Coord(x, y))
        .collect()
}

#[part_one]
fn part_one(grid: Grid<Input>) -> usize {
    let mapping: HashMap<Antenna, Vec<Coord>> = grid
        .positions()
        .map(|p| match grid[p.0][p.1] {
            Input::Antenna(antenna) => Some((antenna, Coord(p.0, p.1))),
            Input::Empty => None,
        })
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .fold(HashMap::new(), |mut m, i| {
            m.entry(i.0).or_default().push(i.1);
            m
        });

    println!("{:?}", mapping);

    let antinodes: Vec<Coord> = mapping
        .values()
        .flat_map(|coords| {
            coords
                .iter()
                .enumerate()
                .flat_map(|(i, c)| coords.iter().skip(i).map(|other_coord| (*c, *other_coord)))
                .flat_map(|(c1, c2)| get_antinode(c1, c2))
        })
        .collect();

    antinodes
        .into_iter()
        .filter(|a| (0..grid.width()).contains(&a.0) && (0..grid.height()).contains(&a.1))
        .count()
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
