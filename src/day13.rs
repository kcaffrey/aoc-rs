use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Track {
    track: Vec<Vec<Path>>,
    carts: BinaryHeap<Reverse<Cart>>,
    cart_waiting_room: BinaryHeap<Reverse<Cart>>,
    positions: HashMap<Position, Direction>,
    crashes: HashSet<Position>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cart {
    position: Position,
    direction: Direction,
    next_turn: Turn,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Path {
    Empty,
    Vertical,
    Horizontal,
    CurveRight,
    CurveLeft,
    Intersection,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Track {
    fn step(&mut self) -> Option<Position> {
        self.crashes.clear();
        let mut first_crash = None;
        while let Some(Reverse(mut cart)) = self.carts.pop() {
            self.positions.remove(&cart.position);
            cart.ride(self.track[cart.position.y][cart.position.x]);
            if self.positions.contains_key(&cart.position) {
                if first_crash.is_none() {
                    first_crash = Some(cart.position);
                }
                self.crashes.insert(cart.position);
                self.blow_up_at(cart.position);
            } else {
                self.positions.insert(cart.position, cart.direction);
                self.cart_waiting_room.push(Reverse(cart));
            }
        }
        std::mem::swap(&mut self.carts, &mut self.cart_waiting_room);
        first_crash
    }

    fn blow_up_at(&mut self, position: Position) {
        self.carts = self
            .carts
            .drain()
            .filter(|Reverse(c)| c.position != position)
            .collect();
        self.cart_waiting_room = self
            .cart_waiting_room
            .drain()
            .filter(|Reverse(c)| c.position != position)
            .collect();
        self.positions.remove(&position);
    }
}

impl Cart {
    fn ride(&mut self, path: Path) {
        let mut x = self.position.x as i32;
        let mut y = self.position.y as i32;
        use self::Path::*;
        match path {
            Vertical => y += self.direction.to_offsets().1,
            Horizontal => x += self.direction.to_offsets().0,
            CurveRight | CurveLeft => {
                self.direction = self.direction.handle_curve(path);
                x += self.direction.to_offsets().0;
                y += self.direction.to_offsets().1;
            }
            Intersection => {
                let turn = self.next_turn();
                self.direction = self.direction.handle_turn(turn);
                x += self.direction.to_offsets().0;
                y += self.direction.to_offsets().1;
            }
            Empty => unreachable!("derailed?"),
        }
        self.position = Position {
            x: x as usize,
            y: y as usize,
        };
    }

    fn next_turn(&mut self) -> Turn {
        let turn = self.next_turn;
        match turn {
            Turn::Left => self.next_turn = Turn::Straight,
            Turn::Straight => self.next_turn = Turn::Right,
            Turn::Right => self.next_turn = Turn::Left,
        }
        turn
    }
}

impl Direction {
    fn to_offsets(&self) -> (i32, i32) {
        use self::Direction::*;
        let dx = if self == &Up || self == &Down {
            0
        } else if self == &Left {
            -1
        } else {
            1
        };
        let dy = if self == &Left || self == &Right {
            0
        } else if self == &Up {
            -1
        } else {
            1
        };
        (dx, dy)
    }

    fn handle_curve(&self, path: Path) -> Direction {
        use self::Direction::*;
        use self::Path::*;
        match path {
            CurveRight => match self {
                Up => Right,
                Right => Up,
                Left => Down,
                Down => Left,
            },
            CurveLeft => match self {
                Up => Left,
                Left => Up,
                Right => Down,
                Down => Right,
            },
            _ => unreachable!(),
        }
    }

    fn handle_turn(&self, turn: Turn) -> Direction {
        use self::Direction::*;
        match turn {
            Turn::Straight => *self,
            Turn::Left => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            Turn::Right => match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.track.len() {
            for x in 0..self.track[y].len() {
                if let Some(direction) = self.positions.get(&Position { x, y }) {
                    write!(
                        f,
                        "{}",
                        match direction {
                            Direction::Up => '^',
                            Direction::Right => '>',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                        }
                    )?;
                } else {
                    write!(f, "{}", self.track[y][x])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Path::Horizontal => '-',
                Path::Vertical => '|',
                Path::CurveLeft => '\\',
                Path::CurveRight => '/',
                Path::Intersection => '+',
                Path::Empty => ' ',
            }
        )
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        match self.position.y.cmp(&other.position.y) {
            Ordering::Equal => self.position.x.cmp(&other.position.x),
            x => x,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Box<Track> {
    use self::Direction::*;
    use self::Path::*;
    let mut carts = BinaryHeap::new();
    let track: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '|' => Vertical,
                    '-' => Horizontal,
                    '/' => CurveRight,
                    '\\' => CurveLeft,
                    '+' => Intersection,
                    '^' | 'v' => {
                        carts.push(Reverse(Cart {
                            position: Position { x, y },
                            direction: if ch == 'v' { Down } else { Up },
                            next_turn: Turn::Left,
                        }));
                        Vertical
                    }
                    '>' | '<' => {
                        carts.push(Reverse(Cart {
                            position: Position { x, y },
                            direction: if ch == '<' { Left } else { Right },
                            next_turn: Turn::Left,
                        }));
                        Horizontal
                    }
                    _ => Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    Box::new(Track {
        track,
        cart_waiting_room: BinaryHeap::with_capacity(carts.len()),
        positions: carts
            .iter()
            .map(|Reverse(c)| (c.position, c.direction))
            .collect(),
        carts,
        crashes: HashSet::new(),
    })
}

#[aoc(day13, part1)]
fn solve_part1(track: &Track) -> Position {
    let mut track = track.clone();
    loop {
        if let Some(crash) = track.step() {
            break crash;
        }
    }
}

#[aoc(day13, part2)]
fn solve_part2(track: &Track) -> Position {
    let mut track = track.clone();
    while track.carts.len() > 1 {
        track.step();
    }
    track.carts.pop().unwrap().0.position
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;

    #[test]
    fn test_parse() {
        let mut track = parse(EXAMPLE);
        assert_eq!(
            Some(Reverse(Cart {
                position: Position { x: 2, y: 0 },
                direction: Direction::Right,
                next_turn: Turn::Left,
            })),
            track.carts.pop()
        );
        assert_eq!(
            Some(Reverse(Cart {
                position: Position { x: 9, y: 3 },
                direction: Direction::Down,
                next_turn: Turn::Left,
            })),
            track.carts.pop()
        );
        assert_eq!(None, track.carts.pop());
        assert_eq!(Path::CurveRight, track.track[0][0]);
        assert_eq!(Path::CurveLeft, track.track[4][0]);
        assert_eq!(Path::Intersection, track.track[2][4]);
        assert_eq!(Path::Vertical, track.track[1][0]);
        assert_eq!(Path::Horizontal, track.track[0][2]);
        assert_eq!(Path::Empty, track.track[1][1]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Position { x: 7, y: 3 }, solve_part1(&parse(EXAMPLE)));
    }

    #[test]
    fn test_part2() {
        let example2 = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        assert_eq!(Position { x: 6, y: 4 }, solve_part2(&parse(example2)));
    }
}
