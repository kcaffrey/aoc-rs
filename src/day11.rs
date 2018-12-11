use rayon::prelude::*;
use std::cmp;
use std::convert::AsRef;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SerialNumber(u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct FuelGrid(Vec<Vec<PowerLevel>>);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct PowerLevel(i32);

fn compute_power(x: u32, y: u32, serial: u32) -> PowerLevel {
    let rack_id = x + 10;
    let mut power_level = (rack_id * y) as i32;
    power_level += serial as i32;
    power_level *= rack_id as i32;
    power_level = (power_level / 100) % 10;
    PowerLevel(power_level - 5)
}

impl FuelGrid {
    fn with_serial(serial: &SerialNumber) -> Self {
        let grid = (1u32..301)
            .into_par_iter()
            .map(|y| {
                (1..=300)
                    .map(move |x| compute_power(x, y, serial.0))
                    .collect()
            })
            .collect();
        FuelGrid(grid)
    }

    fn power_at(&self, coord: Coordinate) -> PowerLevel {
        self.0[(coord.y - 1) as usize][(coord.x - 1) as usize]
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> SerialNumber {
    SerialNumber(input.trim().parse().unwrap())
}

macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coordinate {
            x: $x as u32,
            y: $y as u32,
        }
    };
}

#[aoc(day11, part1)]
fn solve_part1(input: &SerialNumber) -> Coordinate {
    let grid = FuelGrid::with_serial(input);
    (1u32..299)
        .into_par_iter()
        .map(|x| {
            let mut max_power = 0;
            let mut max_coordinate = coord!(1, 1);
            for y in 1..=298 {
                let mut power = 0;
                for i in x..x + 3 {
                    for j in y..y + 3 {
                        power += grid.power_at(coord!(i, j)).0;
                    }
                }
                if power > max_power {
                    max_coordinate = coord!(x, y);
                    max_power = power;
                }
            }
            (max_coordinate, max_power)
        })
        .max_by_key(|(_, power)| *power)
        .unwrap()
        .0
}

#[aoc(day11, part2)]
fn solve_part2(input: &SerialNumber) -> String {
    let grid = FuelGrid::with_serial(input);
    let mut area_sums = vec![vec![0; 301]; 301];
    for x in 1..=300 {
        for y in 1..=300 {
            let mut sum = area_sums[y - 1][x] + area_sums[y][x - 1] - area_sums[y - 1][x - 1];
            sum += grid.power_at(coord!(x, y)).0;
            area_sums[y][x] = sum;
        }
    }
    let (max_coordinate, max_size, _) = (1usize..301)
        .into_par_iter()
        .map(|x| {
            let mut max_power = 0;
            let mut max_size = 0;
            let mut max_coordinate = coord!(1, 1);
            for y in 1..=300 {
                for size in 1..=cmp::min(300 - x + 1, 300 - y + 1) {
                    let (x1, y1) = (x + size - 1, y + size - 1);
                    let power = area_sums[y1][x1] - area_sums[y1][x - 1] - area_sums[y - 1][x1]
                        + area_sums[y - 1][x - 1];
                    if power > max_power {
                        max_power = power;
                        max_size = size;
                        max_coordinate = coord!(x, y);
                    }
                }
            }
            (max_coordinate, max_size, max_power)
        })
        .max_by_key(|(_, _, power)| *power)
        .unwrap();
    format!("{},{},{}", max_coordinate.x, max_coordinate.y, max_size).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(SerialNumber(18), parse("18"));
    }

    #[test]
    fn test_compute_power() {
        assert_eq!(PowerLevel(4), compute_power(3, 5, 8));
        assert_eq!(PowerLevel(-5), compute_power(122, 79, 57));
        assert_eq!(PowerLevel(0), compute_power(217, 196, 39));
        assert_eq!(PowerLevel(4), compute_power(101, 153, 71));
    }

    #[test]
    fn test_part1() {
        assert_eq!(coord!(33, 45), solve_part1(&parse("18")));
        assert_eq!(coord!(21, 61), solve_part1(&parse("42")));
    }

    #[test]
    fn test_part2() {
        assert_eq!("90,269,16", &solve_part2(&parse("18")));
        assert_eq!("232,251,12", &solve_part2(&parse("42")));
    }
}

impl AsRef<SerialNumber> for SerialNumber {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(u32, u32)> for Coordinate {
    fn from(from: (u32, u32)) -> Self {
        Self {
            x: from.0,
            y: from.1,
        }
    }
}
