use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Coordinate<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Ord> Ord for Coordinate<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            x => x,
        }
    }
}

impl<T: Ord> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Display> Display for Coordinate<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
