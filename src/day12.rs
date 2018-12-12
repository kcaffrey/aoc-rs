use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rule {
    pattern: u8,
    result: bool,
}

impl Rule {
    fn from_input(pattern: &str, result: &str) -> Rule {
        let pattern = pattern
            .chars()
            .map(|c| if c == '#' { 1u8 } else { 0u8 })
            .rev()
            .enumerate()
            .map(|(i, b)| b << i)
            .sum();
        let result = result == "#";
        Rule { pattern, result }
    }
}

#[derive(Debug, PartialEq)]
struct State(HashMap<i32, bool>);

#[derive(Debug, PartialEq)]
struct InitialState {
    state: State,
    rules: Vec<Rule>,
}

impl From<&str> for State {
    fn from(from: &str) -> State {
        State(
            from.chars()
                .enumerate()
                .map(|(i, c)| (i as i32, c == '#'))
                .collect(),
        )
    }
}

struct Simulation {
    current: State,
    next: State,
    rules: [bool; 32],
    min: i32,
    max: i32,
    gen: u32,
}

impl From<&InitialState> for Simulation {
    fn from(from: &InitialState) -> Simulation {
        let capacity = from.state.0.len();
        let min = from.state.0.iter().map(|(i, _)| *i).min().unwrap();
        let max = from.state.0.iter().map(|(i, _)| *i).max().unwrap();
        let mut rules = [false; 32];
        from.rules
            .iter()
            .for_each(|r| rules[r.pattern as usize] = r.result);
        Simulation {
            current: State(from.state.0.clone()),
            next: State(HashMap::with_capacity(capacity)),
            rules,
            min,
            max,
            gen: 0,
        }
    }
}

impl Simulation {
    pub fn run(&mut self, num_iters: u32) {
        for _ in 0..num_iters {
            self.update();
        }
    }

    pub fn update(&mut self) {
        for index in self.min - 2..=self.max + 2 {
            let cur_exists = self.current.0.contains_key(&index);
            let val = self.rules[self.pattern_at(index) as usize];
            if val || cur_exists {
                self.next.0.insert(index, val);
                if index < self.min {
                    self.min = index;
                }
                if index > self.max {
                    self.max = index;
                }
            }
        }
        self.gen += 1;
        std::mem::swap(&mut self.next, &mut self.current);
    }

    pub fn plant_index_sum(&self) -> i32 {
        self.current
            .0
            .iter()
            .flat_map(|(i, &val)| if val { Some(i) } else { None })
            .sum()
    }

    fn pattern_at(&self, index: i32) -> u8 {
        (index - 2..=index + 2)
            .map(|i| self.current.0.get(&i).unwrap_or(&false))
            .map(|&b| if b { 1u8 } else { 0u8 })
            .rev()
            .enumerate()
            .map(|(i, b)| b << i)
            .sum()
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Box<InitialState> {
    lazy_static::lazy_static! {
        static ref INITIAL: Regex = Regex::new(r"initial state: ([.#]+)").unwrap();
        static ref RULE: Regex = Regex::new(r"([.#]{5}) => ([.#])").unwrap();
    }
    let state = INITIAL.captures(input).unwrap()[1].into();
    let rules = RULE
        .captures_iter(input)
        .map(|caps| Rule::from_input(&caps[1], &caps[2]))
        .collect();
    Box::new(InitialState { state, rules })
}

#[aoc(day12, part1)]
fn solve_part1(initial_state: &InitialState) -> i32 {
    let mut simulation: Simulation = initial_state.into();
    simulation.run(20);
    simulation.plant_index_sum()
}

#[aoc(day12, part2)]
fn solve_part2(initial_state: &InitialState) -> u64 {
    let mut simulation: Simulation = initial_state.into();
    // At some point the simulation will stabilize into adding the same value over and over.
    // 0 will mean that the simulation has completely stabilized, while non-zero represents a
    // moving pattern.
    // It's possible to enter a cycle (which we don't detect). Thankfully AoC wasn't that evil.
    // Run the simulation until stabilization, and math out the rest.
    simulation.run(80);
    let mut increments = HashSet::with_capacity(5);
    while increments.len() != 1 {
        // Skip a few.
        simulation.run(20);

        // Run 5 iterations, keeping track of how much the index sum changes every generation.
        increments.clear();
        let mut last_sum = simulation.plant_index_sum();
        for _ in 0..5 {
            simulation.update();
            let sum = simulation.plant_index_sum();
            increments.insert(sum - last_sum);
            last_sum = sum;
        }
    }

    (simulation.plant_index_sum() as u64)
        + (50_000_000_000 - simulation.gen as u64) * (*increments.iter().next().unwrap() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_parse() {
        let short_input = "initial state: #..#\n\n...## => #\n..#.. => .";
        let state = State(
            [(0, true), (1, false), (2, false), (3, true)]
                .iter()
                .cloned()
                .collect(),
        );
        let rules = vec![
            Rule {
                pattern: 0b00011,
                result: true,
            },
            Rule {
                pattern: 0b00100,
                result: false,
            },
        ];
        let expected = InitialState { state, rules };
        assert_eq!(Box::new(expected), parse(short_input));
    }

    #[test]
    fn test_part1() {
        assert_eq!(325, solve_part1(&parse(INPUT)));
    }
}
