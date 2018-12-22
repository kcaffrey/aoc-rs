use num_traits::identities::{One, Zero};
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Coordinate { x, y }
    }
}

impl<T: Default> Default for Coordinate<T> {
    fn default() -> Self {
        Coordinate {
            x: Default::default(),
            y: Default::default(),
        }
    }
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

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl<T> Coordinate<T>
where
    T: Add<Output = T> + Sub<Output = T> + One + Zero + Copy,
{
    pub fn up(&self) -> Option<Self> {
        if self.y.is_zero() {
            return None;
        }
        Some(Coordinate {
            x: self.x,
            y: self.y - T::one(),
        })
    }

    pub fn down(&self) -> Option<Self> {
        Some(Coordinate {
            x: self.x,
            y: self.y + T::one(),
        })
    }

    pub fn left(&self) -> Option<Self> {
        if self.x.is_zero() {
            return None;
        }
        Some(Coordinate {
            x: self.x - T::one(),
            y: self.y,
        })
    }

    pub fn right(&self) -> Option<Self> {
        Some(Coordinate {
            x: self.x + T::one(),
            y: self.y,
        })
    }
}

impl<T> Coordinate<T>
where
    T: Ord + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn distance(&self, other: Self) -> T {
        use std::cmp::{max, min};
        max(self.x, other.x) - min(self.x, other.x) + max(self.y, other.y) - min(self.y, other.y)
    }
}
