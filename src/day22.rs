use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Ordering, Reverse};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

type Coordinate = crate::coordinate::Coordinate<usize>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Box<(usize, Coordinate)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"depth: (\d+)\s+target: (\d+),(\d+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let depth = caps[1].parse().unwrap();
    let target = Coordinate::new(caps[2].parse().unwrap(), caps[3].parse().unwrap());
    Box::new((depth, target))
}

#[aoc(day22, part1)]
fn solve_part1(&(depth, target): &(usize, Coordinate)) -> usize {
    let map = Map::new(depth, target);
    let mut sum = 0;
    for y in 0..=target.y {
        for x in 0..=target.x {
            sum += map.0[y][x].risk()
        }
    }
    sum
}

#[aoc(day22, part2)]
fn solve_part2(&(depth, target): &(usize, Coordinate)) -> usize {
    let map = Map::new(depth, target);
    let mut queue: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    let mut distances: HashMap<(Coordinate, Tool), usize> = HashMap::new();
    let origin = Coordinate::new(0, 0);
    queue.push(Reverse(Position {
        coordinate: origin,
        tool: Tool::Torch,
        distance: 0,
        heuristic: target.distance(origin),
    }));
    distances.insert((origin, Tool::Torch), 0);
    while let Some(Reverse(position)) = queue.pop() {
        if distances[&(position.coordinate, position.tool)] < position.distance {
            continue;
        }
        if position.coordinate == target && position.tool == Tool::Torch {
            // Done!
            break;
        }
        map.visit_neighbors(
            position.coordinate,
            position.tool,
            |coordinate, tool, cost| {
                let mut enqueue_neighbor = false;
                let distance = position.distance + cost;
                match distances.entry((coordinate, tool)) {
                    Entry::Occupied(mut e) => {
                        if distance < *e.get() {
                            *e.get_mut() = distance;
                            enqueue_neighbor = true;
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(distance);
                        enqueue_neighbor = true;
                    }
                }
                if enqueue_neighbor {
                    queue.push(Reverse(Position {
                        coordinate,
                        tool,
                        distance,
                        heuristic: distance + coordinate.distance(target),
                    }));
                }
            },
        );
    }
    distances[&(target, Tool::Torch)]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    coordinate: Coordinate,
    tool: Tool,
    distance: usize,
    heuristic: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.heuristic.cmp(&other.heuristic) {
            Ordering::Equal => self.distance.cmp(&other.distance),
            ord => ord,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Terrain {
    fn risk(self) -> usize {
        match self {
            Terrain::Rocky => 0,
            Terrain::Wet => 1,
            Terrain::Narrow => 2,
        }
    }

    fn passable(self, tool: Tool) -> bool {
        match self {
            Terrain::Rocky => tool == Tool::Torch || tool == Tool::ClimbingGear,
            Terrain::Wet => tool == Tool::ClimbingGear || tool == Tool::Neither,
            Terrain::Narrow => tool == Tool::Torch || tool == Tool::Neither,
        }
    }
}

impl Tool {
    fn other_tools(self) -> impl Iterator<Item = Self> {
        [Tool::ClimbingGear, Tool::Torch, Tool::Neither]
            .iter()
            .cloned()
            .filter(move |&t| t != self)
    }
}

type Erosion = usize;
impl From<Erosion> for Terrain {
    fn from(from: Erosion) -> Self {
        match from % 3 {
            0 => Terrain::Rocky,
            1 => Terrain::Wet,
            2 => Terrain::Narrow,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map(Vec<Vec<Terrain>>);

impl Map {
    fn new(depth: usize, target: Coordinate) -> Map {
        let map_multiple = 4;
        let mut erosion = vec![vec![0; target.x * map_multiple]; target.y * map_multiple];
        for x in 0..target.x * map_multiple {
            erosion[0][x] = (x * 16807 + depth) % 20183;
        }
        for (y, row) in erosion.iter_mut().enumerate() {
            row[0] = (y * 48271 + depth) % 20183;
        }
        for y in 1..target.y * map_multiple {
            for x in 1..target.x * map_multiple {
                erosion[y][x] = (erosion[y - 1][x] * erosion[y][x - 1] + depth) % 20183;
                if x == target.x && y == target.y {
                    erosion[y][x] = depth % 20183;
                }
            }
        }
        Map(erosion
            .into_iter()
            .map(|row| row.into_iter().map(Terrain::from).collect())
            .collect())
    }

    fn visit_neighbors<F>(&self, coordinate: Coordinate, tool: Tool, mut f: F)
    where
        F: FnMut(Coordinate, Tool, usize),
    {
        // Visit all neighbors with the same tool.
        [
            coordinate.up(),
            coordinate.left(),
            coordinate.right(),
            coordinate.down(),
        ]
        .iter()
        .cloned()
        .filter_map(|c| c)
        .filter(|&c| c.x < self.0[0].len() && c.y < self.0.len() && self.0[c.y][c.x].passable(tool))
        .for_each(|c| f(c, tool, 1));

        // Visit other tools for the same coordinate.
        tool.other_tools()
            .filter(|&t| self.0[coordinate.y][coordinate.x].passable(t))
            .for_each(|t| f(coordinate, t, 7));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(114, solve_part1(&(510, Coordinate::new(10, 10))));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45, solve_part2(&(510, Coordinate::new(10, 10))));
    }
}
