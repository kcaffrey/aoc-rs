use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Clone)]
struct Landscape {
    acres: Vec<Vec<Acre>>,
    neighbor_counts: Vec<Vec<(u32, u32)>>,
    change_list: Vec<(usize, usize, Acre)>,
    rows: usize,
    cols: usize,
}

impl From<char> for Acre {
    fn from(from: char) -> Self {
        match from {
            '.' => Acre::Open,
            '#' => Acre::Lumberyard,
            '|' => Acre::Trees,
            _ => unreachable!(),
        }
    }
}

impl Landscape {
    pub fn tick(&mut self) {
        self.change_list.clear();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cur = self.acres[row][col];
                let next = match (cur, self.neighbor_counts[row][col]) {
                    (Acre::Open, (trees, _)) if trees >= 3 => Acre::Trees,
                    (Acre::Trees, (_, yards)) if yards >= 3 => Acre::Lumberyard,
                    (Acre::Lumberyard, (trees, yards)) if trees < 1 || yards < 1 => Acre::Open,
                    (cur, _) => cur,
                };
                if next != cur {
                    self.change_list.push((row, col, next));
                }
            }
        }
        for &(row, col, next) in &self.change_list {
            self.acres[row][col] = next;
            for r in row.saturating_sub(1)..=row + 1 {
                for c in col.saturating_sub(1)..=col + 1 {
                    if r < self.rows && c < self.cols && (r != row || c != col) {
                        self.neighbor_counts[r][c] = match (next, self.neighbor_counts[r][c]) {
                            (Acre::Trees, (trees, yards)) => (trees + 1, yards),
                            (Acre::Lumberyard, (trees, yards)) => (trees - 1, yards + 1),
                            (Acre::Open, (trees, yards)) => (trees, yards - 1),
                        };
                    }
                }
            }
        }
    }

    fn count_neighbors(grid: &Vec<Vec<Acre>>, row: usize, col: usize) -> (u32, u32) {
        let (mut trees, mut lumberyards) = (0, 0);
        for r in row.saturating_sub(1)..=std::cmp::min(row + 1, grid.len() - 1) {
            for c in col.saturating_sub(1)..=std::cmp::min(col + 1, grid[0].len() - 1) {
                if r == row && c == col {
                    continue;
                }
                match grid[r][c] {
                    Acre::Trees => trees += 1,
                    Acre::Lumberyard => lumberyards += 1,
                    Acre::Open => {}
                }
            }
        }
        (trees, lumberyards)
    }

    fn resource_value(&self) -> u32 {
        let (mut trees, mut yards) = (0, 0);
        for row in &self.acres {
            for acre in row {
                match acre {
                    Acre::Trees => trees += 1,
                    Acre::Lumberyard => yards += 1,
                    _ => {}
                }
            }
        }
        trees * yards
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Box<Landscape> {
    let acres = input
        .trim()
        .lines()
        .map(|line| line.chars().map(Acre::from).collect())
        .collect::<Vec<Vec<Acre>>>();
    let (rows, cols) = (acres.len(), acres[0].len());
    let mut neighbor_counts = vec![vec![(0, 0); cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            neighbor_counts[row][col] = Landscape::count_neighbors(&acres, row, col);
        }
    }
    Box::new(Landscape {
        change_list: Vec::with_capacity(rows * cols),
        acres,
        neighbor_counts,
        rows,
        cols,
    })
}

#[aoc(day18, part1)]
fn solve_part1(landscape: &Landscape) -> u32 {
    let mut landscape = landscape.clone();
    for _ in 0..10 {
        landscape.tick();
    }
    landscape.resource_value()
}

#[aoc(day18, part2)]
fn solve_part2(landscape: &Landscape) -> u32 {
    let mut landscape = landscape.clone();

    // Try to find a cycle (it's assumed that if a pair of resource values is seen twice, we have a
    // cycle. Not always true, but should be safe enough).
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut prev = landscape.resource_value();
    let mut cur;
    let mut initial_length = 0;
    loop {
        initial_length += 1;
        landscape.tick();
        cur = landscape.resource_value();
        if !seen.insert((cur, prev)) {
            break;
        }
        prev = cur;
    }

    // Now find the cycle length by looping until the first element in the cycle is reached again.
    let mut cycle = vec![landscape.resource_value()];
    landscape.tick();
    loop {
        let resource_value = landscape.resource_value();
        if resource_value == cycle[0] {
            break;
        }
        cycle.push(resource_value);
        landscape.tick();
    }

    // Return the resource value in the cycle for the billionth iteration.
    cycle[(1_000_000_000 - initial_length) % cycle.len()]
}

use std::fmt::{self, Display, Formatter};
impl Display for Landscape {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.acres {
            for col in row {
                let ch = match col {
                    Acre::Open => '.',
                    Acre::Lumberyard => '#',
                    Acre::Trees => '|',
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn test_parse() {
        let landscape = parse(EXAMPLE);
        use self::Acre::*;
        assert_eq!(
            &landscape.acres[0][..],
            &[Open, Lumberyard, Open, Lumberyard, Open, Open, Open, Trees, Lumberyard, Open]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(1147, solve_part1(&parse(EXAMPLE)));
    }
}
