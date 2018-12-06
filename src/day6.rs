use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    x: i16,
    y: i16,
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<Coord> {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    input
        .lines()
        .filter_map(|line| {
            let caps = re.captures(line)?;
            Some(Coord {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            })
        }).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Coord]) -> u32 {
    let minx = input.iter().map(|c| c.x).min().unwrap();
    let maxx = input.iter().map(|c| c.y).max().unwrap();
    let miny = input.iter().map(|c| c.x).min().unwrap();
    let maxy = input.iter().map(|c| c.y).max().unwrap();
    let mut infinites = HashSet::new();
    let mut counts: HashMap<usize, u32> = HashMap::new();
    for x in minx..=maxx {
        for y in miny..=maxy {
            let mut closest: Option<usize> = None;
            let mut best_distance = i16::max_value();
            for (index, c) in input.iter().enumerate() {
                let distance = (x - c.x).abs() + (y - c.y).abs();
                if distance < best_distance {
                    closest = Some(index);
                    best_distance = distance;
                } else if closest.is_some() && distance == best_distance {
                    closest = None;
                }
            }
            if let Some(closest) = closest {
                if (x == minx || y == miny || x == maxx || y == maxy)
                    && !infinites.contains(&closest)
                {
                    infinites.insert(closest);
                    counts.remove(&closest);
                } else {
                    *counts.entry(closest).or_default() += 1;
                }
            }
        }
    }
    counts.values().max().cloned().unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Coord]) -> u32 {
    let minx = input.iter().map(|c| c.x).min().unwrap();
    let maxx = input.iter().map(|c| c.y).max().unwrap();
    let miny = input.iter().map(|c| c.x).min().unwrap();
    let maxy = input.iter().map(|c| c.y).max().unwrap();
    let mut count = 0;
    for x in minx..=maxx {
        for y in miny..=maxy {
            let total_distance: i16 = input
                .iter()
                .map(|c| (x - c.x).abs() + (y - c.y).abs())
                .sum();
            if total_distance < 10000 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn test_parse() {
        assert_eq!(
            &[
                Coord { x: 1, y: 1 },
                Coord { x: 1, y: 6 },
                Coord { x: 8, y: 3 }
            ],
            &parse(INPUT)[..3]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(17, solve_part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        // Not bothering with a test since it means overwriting a constant that is annoying to set
        // using the AOC runner.
    }
}
