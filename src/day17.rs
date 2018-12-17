use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::VecDeque;

type Coordinate = crate::coordinate::Coordinate<usize>;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Cell {
    Sand,
    Clay,
    WetSand,
    Water,
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    offsetx: usize,
    offsety: usize,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Box<Grid> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([xy])=(\d+),\s*[xy]=(\d+)\.\.(\d+)").unwrap();
    }
    let mut horizontal_lines = vec![];
    let mut vertical_lines = vec![];
    for caps in RE.captures_iter(input) {
        let line: (usize, usize, usize) = (
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        );
        match &caps[1] {
            "x" => vertical_lines.push(line),
            "y" => horizontal_lines.push(line),
            _ => unreachable!(),
        }
    }
    let mut miny = horizontal_lines.iter().map(|l| l.0).min().unwrap();
    let mut minx = horizontal_lines.iter().map(|l| l.1).min().unwrap();
    miny = cmp::min(miny, vertical_lines.iter().map(|l| l.1).min().unwrap());
    minx = cmp::min(minx, vertical_lines.iter().map(|l| l.0).min().unwrap()) - 1;
    let mut maxy = horizontal_lines.iter().map(|l| l.0).max().unwrap();
    let mut maxx = horizontal_lines.iter().map(|l| l.2).max().unwrap();
    maxy = cmp::max(maxy, vertical_lines.iter().map(|l| l.2).max().unwrap());
    maxx = cmp::max(maxx, vertical_lines.iter().map(|l| l.0).max().unwrap()) + 1;

    let mut cells = vec![vec![Cell::Sand; maxx - minx + 1]; maxy - miny + 1];
    for line in horizontal_lines {
        for x in line.1..=line.2 {
            cells[line.0 - miny][x - minx] = Cell::Clay;
        }
    }
    for line in vertical_lines {
        for y in line.1..=line.2 {
            cells[y - miny][line.0 - minx] = Cell::Clay;
        }
    }
    Box::new(Grid {
        cells,
        offsetx: minx,
        offsety: miny,
    })
}

fn run_water(grid: &mut Grid) -> (u32, u32) {
    let mut queue = VecDeque::new();
    // There are two cases at the beginning - clay or sand immediately before the water.
    // If clay, we create two sources, otherwise just one.
    // This is only necessary because we compute the counts as we go rather than tallying at the
    // end.
    let sourcex = 500 - grid.offsetx;
    match grid.cells[0][sourcex] {
        Cell::Sand => {
            queue.push_back(Coordinate { y: 0, x: sourcex });
        }
        Cell::Clay => {
            let mut x = sourcex;
            while grid.cells[0][x] == Cell::Clay {
                x -= 1;
            }
            queue.push_back(Coordinate { y: 0, x });
            let mut x = sourcex;
            while grid.cells[0][x] == Cell::Clay {
                x += 1;
            }
            queue.push_back(Coordinate { y: 0, x });
        }
        _ => unreachable!(),
    }

    let mut tile_count = 0;
    let mut water_count = 0;
    while let Some(source) = queue.pop_front() {
        // Make sand wet until we hit the bottom or non-sand.
        let (x, mut y) = (source.x, source.y);
        while y < grid.cells.len() && grid.cells[y][x] == Cell::Sand {
            grid.cells[y][x] = Cell::WetSand;
            tile_count += 1;
            y += 1;
        }

        // If we hit the bottom, we are done with this source- it leaks out.
        if y >= grid.cells.len() {
            continue;
        }

        // If we hit flowing water, we already know we can't fill here and are already done.
        if grid.cells[y][x] == Cell::WetSand {
            continue;
        }

        let mut new_type = Cell::Water;
        while new_type == Cell::Water && y > 0 {
            y -= 1;

            // If we hit clay in both directions without encountering a hole, make water and move up.
            // If we find a hole, create wet sand and create a source at the hole.
            let spread_water = |startx, direction: isize| {
                let mut x = startx;
                loop {
                    // First check to see if we hit a wall, in which case we know this direction is done.
                    let nextx = (x as isize + direction) as usize;
                    if grid.cells[y][nextx] == Cell::Clay {
                        return (x, None);
                    }
                    x = nextx;

                    // Now check for a hole - if the tile under us is not our expected floor,
                    // make a source.
                    if grid.cells[y + 1][x] != Cell::Water && grid.cells[y + 1][x] != Cell::Clay {
                        return (x, Some(Coordinate { x, y: y + 1 }));
                    }
                }
            };
            let (minx, source_left) = spread_water(source.x, -1);
            let (maxx, source_right) = spread_water(source.x, 1);

            // If we found a hole on either side, queue new sources and mark the row as running
            // water.
            for s in &[source_left, source_right] {
                if let Some(s) = s {
                    queue.push_back(*s);
                    new_type = Cell::WetSand;
                }
            }

            // Fill in the row with the new water type.
            for x in minx..=maxx {
                if grid.cells[y][x] == Cell::Sand {
                    tile_count += 1;
                }
                if grid.cells[y][x] != Cell::Water && new_type == Cell::Water {
                    water_count += 1;
                }
                grid.cells[y][x] = new_type;
            }
        }
    }

    (tile_count, water_count)
}

#[aoc(day17, part1)]
fn solve_part1(grid: &Grid) -> u32 {
    let mut grid = grid.clone();
    run_water(&mut grid).0
}

#[aoc(day17, part2)]
fn solve_part2(grid: &Grid) -> u32 {
    let mut grid = grid.clone();
    run_water(&mut grid).1
}

use std::fmt::{self, Display, Formatter};
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for _ in self.offsetx - 1..500 {
            write!(f, ".")?;
        }
        write!(f, "+")?;
        for _ in 501..(self.cells[0].len() + self.offsetx + 1) {
            write!(f, ".")?;
        }
        writeln!(f)?;
        for y in 0..self.cells.len() {
            write!(f, ".")?;
            for x in 0..self.cells[y].len() {
                match self.cells[y][x] {
                    Cell::Sand => write!(f, ".")?,
                    Cell::Clay => write!(f, "#")?,
                    Cell::WetSand => write!(f, "|")?,
                    Cell::Water => write!(f, "~")?,
                }
            }
            writeln!(f, ".")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test_parse() {
        let grid = &parse(EXAMPLE);
        assert_eq!(grid.offsetx, 494);
        assert_eq!(grid.offsety, 1);
        assert_eq!(grid.cells.len(), 13);
        assert_eq!(grid.cells[0].len(), 14);
        use super::Cell::*;
        assert_eq!(&grid.cells[1][1..6], &[Clay, Sand, Sand, Clay, Sand]);
        assert_eq!(&grid.cells[6][1..6], &[Clay, Clay, Clay, Clay, Clay]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(57, solve_part1(&parse(EXAMPLE)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, solve_part2(&parse(EXAMPLE)));
    }
}
