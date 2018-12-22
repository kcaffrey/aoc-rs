use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day20)]
fn parse(input: &str) -> Vec<u8> {
    input[1..input.len() - 1].as_bytes().into()
}

lazy_static! {
    static ref DIRECTIONS: HashMap<u8, Point> = [
        (b'N', Point::new(0, -1)),
        (b'E', Point::new(1, 0)),
        (b'S', Point::new(0, 1)),
        (b'W', Point::new(-1, 0))
    ]
    .iter()
    .cloned()
    .collect();
}

type Point = crate::coordinate::Coordinate<i32>;

#[derive(Debug)]
struct Map(HashMap<Point, Room>);

#[derive(Debug, Default)]
struct Room {
    neighbors: HashSet<Point>,
}

fn build_map(pattern: &[u8]) -> Map {
    let mut map = Map::new();
    let mut stack = vec![];
    let mut cur = Point::new(0, 0);
    for ch in pattern {
        match ch {
            b'W' | b'E' | b'N' | b'S' => {
                let next = cur + DIRECTIONS[&ch];
                map.link_rooms(cur, next);
                cur = next;
            }
            b'(' => stack.push(cur),
            b'|' => cur = *stack.last().unwrap(),
            b')' => {
                stack.pop();
            }
            _ => unreachable!(),
        }
    }
    map
}

fn build_distances(map: &Map) -> HashMap<Point, u32> {
    let mut distances = HashMap::new();
    let mut queue = vec![(Point::new(0, 0), 0)];
    let mut visited = HashSet::new();
    visited.insert(Point::new(0, 0));
    while let Some((cur, distance)) = queue.pop() {
        distances.insert(cur, distance);
        for neighbor in map.neighbors(cur) {
            if visited.insert(neighbor) {
                queue.push((neighbor, distance + 1));
            }
        }
    }
    distances
}

impl Map {
    fn new() -> Self {
        Map(HashMap::new())
    }

    fn link_rooms(&mut self, a: Point, b: Point) {
        self.0.entry(a).or_default().neighbors.insert(b);
        self.0.entry(b).or_default().neighbors.insert(a);
    }

    fn neighbors<'a>(&'a self, room: Point) -> impl Iterator<Item = Point> + 'a {
        self.0[&room].neighbors.iter().cloned()
    }
}

#[aoc(day20, part1)]
fn solve_part1(pattern: &[u8]) -> u32 {
    let map = build_map(pattern);
    *build_distances(&map).values().max().unwrap()
}

#[aoc(day20, part2)]
fn solve_part2(pattern: &[u8]) -> usize {
    let map = build_map(pattern);
    build_distances(&map)
        .values()
        .filter(|&&v| v >= 1000)
        .count()
}

use std::fmt::{self, Display, Formatter};
impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let minx = self.0.keys().map(|p| p.x).min().unwrap();
        let miny = self.0.keys().map(|p| p.y).min().unwrap();
        let maxx = self.0.keys().map(|p| p.x).max().unwrap();
        let maxy = self.0.keys().map(|p| p.y).max().unwrap();
        writeln!(f, "({}, {}) - ({}, {})", minx, miny, maxx, maxy)?;
        for y in miny..=maxy {
            for x in minx..=maxx {
                match self.0.get(&Point::new(x, y)) {
                    None => write!(f, "##")?,
                    Some(room) => {
                        if room.neighbors.contains(&Point::new(x, y - 1)) {
                            write!(f, "#-")?;
                        } else {
                            write!(f, "##")?;
                        }
                    }
                }
            }
            writeln!(f, "#")?;
            for x in minx..=maxx {
                match self.0.get(&Point::new(x, y)) {
                    None => write!(f, "##")?,
                    Some(room) => {
                        let r = if x == 0 && y == 0 { "X" } else { "." };
                        if room.neighbors.contains(&Point::new(x - 1, y)) {
                            write!(f, "|{}", r)?;
                        } else {
                            write!(f, "#{}", r)?;
                        }
                    }
                }
            }
            writeln!(f, "#")?;
        }
        write!(f, "{}", "##".repeat((maxx - minx + 1).abs() as usize))?;
        writeln!(f, "#")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            23,
            solve_part1(b"ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))")
        );
        assert_eq!(
            31,
            solve_part1(b"WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))")
        );
    }
}
