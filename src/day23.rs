use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::ops::{Add, Div};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Bot {
    pos: Position,
    strength: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cuboid(Position, Position);

static ORIGIN: Position = Position { x: 0, y: 0, z: 0 };

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Bot> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    }
    RE.captures_iter(input)
        .filter_map(|caps| {
            Some(Bot {
                pos: Position {
                    x: caps[1].parse().ok()?,
                    y: caps[2].parse().ok()?,
                    z: caps[3].parse().ok()?,
                },
                strength: caps[4].parse().ok()?,
            })
        })
        .collect()
}

#[aoc(day23, part1)]
fn solve_part1(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|bot| bot.strength).unwrap();
    bots.iter()
        .filter(|&bot| bot.pos.distance(strongest.pos) <= strongest.strength)
        .count()
}

#[aoc(day23, part2)]
fn solve_part2(bots: &[Bot]) -> i64 {
    let mut queue = BinaryHeap::with_capacity(5000);
    let universe = Cuboid::from(bots);
    queue.push((bots.len(), Reverse(universe)));
    while let Some((_, Reverse(cuboid))) = queue.pop() {
        if cuboid.0 == cuboid.1 {
            return cuboid.0.distance(ORIGIN);
        }
        queue.extend(
            cuboid
                .subdivide()
                .map(|c| (c.num_bots_intersecting(bots), Reverse(c))),
        );
    }
    unreachable!()
}

impl<'a, T: IntoIterator<Item = &'a Bot>> From<T> for Cuboid {
    fn from(f: T) -> Cuboid {
        let mut max = 0;
        for bot in f {
            max = std::cmp::max(max, bot.pos.x.abs() + bot.strength);
            max = std::cmp::max(max, bot.pos.y.abs() + bot.strength);
            max = std::cmp::max(max, bot.pos.z.abs() + bot.strength);
        }
        // Find next power of 2.
        let mut i = 1;
        while i < max {
            i *= 2;
        }
        Cuboid(Position::new(-i, -i, -i), Position::new(i, i, i))
    }
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Position { x, y, z }
    }

    fn distance(&self, other: Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Cuboid {
    fn subdivide(self) -> impl Iterator<Item = Cuboid> {
        let (a, b) = (self.0, self.1);
        let c = (self.0 + self.1) / 2;
        if a == c || b == c {
            return self
                .corners()
                .iter()
                .map(|&c| Cuboid(c, c))
                .collect::<Vec<_>>()
                .into_iter();
        }
        vec![
            Cuboid(Position::new(a.x, a.y, a.z), c),
            Cuboid(Position::new(b.x, a.y, a.z), c),
            Cuboid(Position::new(a.x, b.y, a.z), c),
            Cuboid(Position::new(a.x, a.y, b.z), c),
            Cuboid(Position::new(b.x, b.y, a.z), c),
            Cuboid(Position::new(b.x, a.y, b.z), c),
            Cuboid(Position::new(a.x, b.y, b.z), c),
            Cuboid(Position::new(b.x, b.y, b.z), c),
        ]
        .into_iter()
    }

    fn num_bots_intersecting(self, bots: &[Bot]) -> usize {
        bots.iter().filter(|&&bot| self.intersects(bot)).count()
    }

    fn intersects(self, bot: Bot) -> bool {
        let projected = Position {
            x: clamp(bot.pos.x, self.0.x, self.1.x),
            y: clamp(bot.pos.y, self.0.y, self.1.y),
            z: clamp(bot.pos.z, self.0.z, self.1.z),
        };
        bot.contains(projected)
    }

    fn distance_to_origin(self) -> i64 {
        self.corners()
            .iter()
            .map(|c| c.distance(ORIGIN))
            .min()
            .unwrap()
    }

    fn corners(self) -> [Position; 8] {
        [
            Position::new(self.0.x, self.0.y, self.0.z),
            Position::new(self.1.x, self.0.y, self.0.z),
            Position::new(self.0.x, self.1.y, self.0.z),
            Position::new(self.0.x, self.0.y, self.1.z),
            Position::new(self.1.x, self.1.y, self.0.z),
            Position::new(self.1.x, self.0.y, self.1.z),
            Position::new(self.0.x, self.1.y, self.1.z),
            Position::new(self.1.x, self.1.y, self.1.z),
        ]
    }
}

impl Bot {
    fn contains(self, p: Position) -> bool {
        p.distance(self.pos) <= self.strength
    }
}

fn clamp(p: i64, a: i64, b: i64) -> i64 {
    use std::cmp::{max, min};
    let (a, b) = (min(a, b), max(a, b));
    if p >= a && p <= b {
        p
    } else if p < a {
        a
    } else {
        b
    }
}

impl Ord for Cuboid {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.distance(self.1).cmp(&other.0.distance(other.1)) {
            Ordering::Equal => self.distance_to_origin().cmp(&other.distance_to_origin()),
            x => x,
        }
    }
}

impl PartialOrd for Cuboid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div<i64> for Position {
    type Output = Self;

    fn div(self, scalar: i64) -> Position {
        Position {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    static EXAMPLE2: &str = "
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

    #[test]
    fn test_part1() {
        assert_eq!(7, solve_part1(&parse(EXAMPLE1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, solve_part2(&parse(EXAMPLE2)));
    }
}
