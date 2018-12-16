use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Coordinate = crate::coordinate::Coordinate<usize>;

#[derive(Debug, PartialEq, Eq)]
struct Unit {
    kind: UnitKind,
    health: u32,
    strength: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnitKind {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Unit(UnitKind),
}

#[derive(Debug, PartialEq)]
struct Grid(Vec<Vec<Cell>>);

#[derive(Debug)]
struct Cave {
    units: Vec<Unit>,
    unit_positions: HashMap<Coordinate, usize>,
    cells: Vec<Vec<Cell>>,
    round: u32,
    num_elves: u32,
    num_goblins: u32,
}

impl From<char> for Cell {
    fn from(from: char) -> Self {
        match from {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'E' => Cell::Unit(UnitKind::Elf),
            'G' => Cell::Unit(UnitKind::Goblin),
            _ => unreachable!(),
        }
    }
}

impl Cave {
    fn new(cells: Vec<Vec<Cell>>, elf_strength: u32) -> Self {
        let mut units = vec![];
        let mut unit_positions = HashMap::new();
        let mut num_elves = 0;
        let mut num_goblins = 0;
        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Unit(kind) = cell {
                    let mut strength = 3;
                    match kind {
                        UnitKind::Goblin => num_goblins += 1,
                        UnitKind::Elf => {
                            num_elves += 1;
                            strength = elf_strength;
                        }
                    }
                    units.push(Unit {
                        kind: *kind,
                        health: 200,
                        strength,
                    });
                    unit_positions.insert(Coordinate { x, y }, units.len() - 1);
                }
            }
        }
        Cave {
            units,
            unit_positions,
            cells,
            round: 0,
            num_elves,
            num_goblins,
        }
    }

    fn tick(&mut self) {
        let mut waiting_units = self.collect_waiting_units();
        let mut move_happened = false;
        let mut unit_died = false;
        while let Some(Reverse((mut coord, index))) = waiting_units.pop() {
            if self.num_elves == 0 || self.num_goblins == 0 {
                return;
            }
            if self.units[index].health == 0 {
                // Unit died, skip it.
                continue;
            }
            let kind = self.units[index].kind;
            if let Some(movement) = self.find_move(coord, kind) {
                self.unit_positions.remove(&coord);
                self.unit_positions.insert(movement, index);
                self.cells[coord.y][coord.x] = Cell::Empty;
                self.cells[movement.y][movement.x] = Cell::Unit(kind);
                coord = movement;
                move_happened = true;
            }
            if let Some((attack_coord, unit_index)) = self.find_attack(coord, kind) {
                unit_died = self.do_attack(index, unit_index, attack_coord) || unit_died;
            }
        }
        self.round += 1;

        // If a move didn't happen, then a unit needs to die before something else will happen.
        // Find pairs of attacks and do attacks until something dies.
        if !move_happened && !unit_died {
            let mut attack_pairs = vec![];
            let mut waiting_units = self.collect_waiting_units();
            while let Some(Reverse((coord, index))) = waiting_units.pop() {
                let kind = self.units[index].kind;
                if let Some((attacked_coord, unit_index)) = self.find_attack(coord, kind) {
                    attack_pairs.push((index, (unit_index, attacked_coord)));
                }
            }

            loop {
                // First check to see if anything will die, and end early so we can simulate a full round.
                let mut temp_health: Vec<_> = self.units.iter().map(|u| u.health).collect();
                for (attacker_index, (target_index, _)) in attack_pairs.iter().cloned() {
                    let strength = self.units[attacker_index].strength;
                    temp_health[target_index] = temp_health[target_index].saturating_sub(strength);
                    if temp_health[target_index] == 0 {
                        return;
                    }
                }

                // Now actually deal damage for this round.
                for (attacker_index, (target_index, target_coord)) in attack_pairs.iter().cloned() {
                    self.do_attack(attacker_index, target_index, target_coord);
                }
                self.round += 1;
            }
        }
    }

    fn collect_waiting_units(&self) -> BinaryHeap<Reverse<(Coordinate, usize)>> {
        self.unit_positions
            .iter()
            .map(|(&c, &i)| Reverse((c, i)))
            .collect()
    }

    fn find_move(&self, coord: Coordinate, kind: UnitKind) -> Option<Coordinate> {
        type Distance = usize;
        type FirstMove = Coordinate;
        let mut open: BinaryHeap<Reverse<(Distance, Coordinate, FirstMove)>> = BinaryHeap::new();
        let mut closed = HashSet::new();
        closed.insert(coord);
        for neighbor in self.neighbors(coord) {
            if let Cell::Unit(other_kind) = self.cells[neighbor.y][neighbor.x] {
                if kind != other_kind {
                    // If we are already next to a unit of the other kind, don't move.
                    return None;
                }
            } else if self.cells[neighbor.y][neighbor.x] == Cell::Empty {
                open.push(Reverse((1, neighbor, neighbor)));
                closed.insert(neighbor);
            }
        }
        while let Some(Reverse((distance, next_coord, first_move))) = open.pop() {
            for neighbor in self.neighbors(next_coord) {
                if closed.contains(&neighbor) {
                    continue;
                }
                if let Cell::Unit(other_kind) = self.cells[neighbor.y][neighbor.x] {
                    if kind != other_kind {
                        // We found a reachable unit of the other kind, so return the first move in that direction.
                        return Some(first_move);
                    }
                } else if self.cells[neighbor.y][neighbor.x] == Cell::Empty {
                    open.push(Reverse((distance + 1, neighbor, first_move)));
                    closed.insert(neighbor);
                }
            }
        }
        None
    }

    fn find_attack(&self, coord: Coordinate, kind: UnitKind) -> Option<(Coordinate, usize)> {
        let mut attack = None;
        let mut min_health = u32::max_value();
        for neighbor in self.neighbors(coord) {
            if let Cell::Unit(other_kind) = self.cells[neighbor.y][neighbor.x] {
                if kind != other_kind {
                    let index = self.unit_positions[&neighbor];
                    if self.units[index].health < min_health || attack.is_none() {
                        attack = Some((neighbor, index));
                        min_health = self.units[index].health;
                    }
                }
            }
        }
        attack
    }

    fn do_attack(
        &mut self,
        attacker_index: usize,
        target_index: usize,
        target_coord: Coordinate,
    ) -> bool {
        let strength = self.units[attacker_index].strength;
        let attacked = &mut self.units[target_index];
        attacked.health = attacked.health.saturating_sub(strength);
        if attacked.health == 0 {
            self.cells[target_coord.y][target_coord.x] = Cell::Empty;
            self.unit_positions.remove(&target_coord);
            match attacked.kind {
                UnitKind::Elf => self.num_elves -= 1,
                UnitKind::Goblin => self.num_goblins -= 1,
            }
            return true;
        }
        false
    }

    fn neighbors(&self, coord: Coordinate) -> impl Iterator<Item = Coordinate> {
        let xlen = self.cells[0].len();
        let ylen = self.cells.len();
        [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .iter()
            .cloned()
            .filter_map(move |(dx, dy)| {
                let x = coord.x as isize + dx;
                let y = coord.y as isize + dy;
                if x < 0 || y < 0 {
                    return None;
                }
                let (x, y) = (x as usize, y as usize);
                if y >= ylen || x >= xlen {
                    return None;
                }
                Some(Coordinate { x, y })
            })
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            let mut healths = vec![];
            for (x, cell) in row.iter().enumerate() {
                write!(
                    f,
                    "{}",
                    match cell {
                        Cell::Empty => ".",
                        Cell::Wall => "#",
                        Cell::Unit(kind) => {
                            let ch = if kind == &UnitKind::Elf { "E" } else { "G" };
                            let index = self.unit_positions[&Coordinate { y, x }];
                            healths.push(format!("{}: {}", ch, self.units[index].health));
                            ch
                        }
                    }
                )?;
            }
            let healths = healths.join(", ");
            writeln!(f, "\t\t{}", healths)?;
        }
        Ok(())
    }
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Box<Grid> {
    Box::new(Grid(
        input
            .lines()
            .map(|line| line.chars().map(Cell::from).collect())
            .collect(),
    ))
}

#[aoc(day15, part1)]
fn solve_part1(grid: &Grid) -> u32 {
    let mut cave = Cave::new(grid.0.clone(), 3);
    while cave.num_elves > 0 && cave.num_goblins > 0 {
        cave.tick();
    }
    let hitpoints: u32 = cave.units.iter().map(|u| u.health).sum();
    hitpoints * cave.round
}

#[aoc(day15, part2)]
fn solve_part2(grid: &Grid) -> u32 {
    for elf_strength in 4.. {
        let mut cave = Cave::new(grid.0.clone(), elf_strength);
        let initial_elves = cave.num_elves;
        while cave.num_elves == initial_elves && cave.num_goblins > 0 {
            cave.tick();
        }
        if cave.num_elves == initial_elves && cave.num_goblins == 0 {
            let hitpoints: u32 = cave.units.iter().map(|u| u.health).sum();
            return hitpoints * cave.round;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLES: &[&str] = &[
        "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######",
        "#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######",
        "#######\n#E..EG#\n#.#G.E#\n#E.##E#\n#G..#.#\n#..E#.#\n#######",
        "#######\n#E.G#.#\n#.#G..#\n#G.#.G#\n#G..#.#\n#...E.#\n#######",
        "#######\n#.E...#\n#.#..G#\n#.###.#\n#E#G#G#\n#...#G#\n#######",
        "#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########",
    ];

    #[test]
    fn test_parse() {
        let cells = parse("###\n#.E\n#G.").0;
        assert_eq!(
            cells,
            vec![
                vec![Cell::Wall; 3],
                vec![Cell::Wall, Cell::Empty, Cell::Unit(UnitKind::Elf)],
                vec![Cell::Wall, Cell::Unit(UnitKind::Goblin), Cell::Empty]
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(27730, solve_part1(&parse(EXAMPLES[0])));
        assert_eq!(36334, solve_part1(&parse(EXAMPLES[1])));
        assert_eq!(39514, solve_part1(&parse(EXAMPLES[2])));
        assert_eq!(27755, solve_part1(&parse(EXAMPLES[3])));
        assert_eq!(28944, solve_part1(&parse(EXAMPLES[4])));
        assert_eq!(18740, solve_part1(&parse(EXAMPLES[5])));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4988, solve_part2(&parse(EXAMPLES[0])));
        assert_eq!(31284, solve_part2(&parse(EXAMPLES[2])));
        assert_eq!(3478, solve_part2(&parse(EXAMPLES[3])));
        assert_eq!(6474, solve_part2(&parse(EXAMPLES[4])));
        assert_eq!(1140, solve_part2(&parse(EXAMPLES[5])));
    }
}
