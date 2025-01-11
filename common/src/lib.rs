use advent::prelude::*;

#[derive(Debug)]
pub struct Digit(pub usize);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|d| Self(d.to_string().parse().unwrap()))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coord(pub usize, pub usize);

pub trait GetCoord<T> {
    fn get_coord(&self, coord: Coord) -> Option<&T>;
}

impl<T> GetCoord<T> for Grid<T> {
    fn get_coord(&self, coord: Coord) -> Option<&T> {
        if coord.1 >= self.height() || coord.0 >= self.width() {
            None
        } else {
            Some(&self[coord.0][coord.1])
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn advance(&self, coord: Coord, bounds: Coord) -> Option<Coord> {
        match self {
            Self::N => (coord.1 < bounds.1).then(|| Coord(coord.0, coord.1 + 1)),
            Self::E => (coord.0 < bounds.0).then(|| Coord(coord.0 + 1, coord.1)),
            Self::S => (coord.1 > 0).then(|| Coord(coord.0, coord.1 - 1)),
            Self::W => (coord.0 > 0).then(|| Coord(coord.0 - 1, coord.1)),
        }
    }
}
