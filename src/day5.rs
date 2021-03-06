use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit {
    kind: char,
    polarity: bool,
}

impl Unit {
    fn reacts(&self, other: &Unit) -> bool {
        self.kind == other.kind && self.polarity != other.polarity
    }
}

impl From<char> for Unit {
    fn from(c: char) -> Unit {
        Unit {
            kind: c.to_lowercase().next().unwrap(),
            polarity: c.is_uppercase(),
        }
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vec<Unit> {
    input.trim().par_chars().map(Unit::from).collect()
}

fn react(polymer: &[Unit], skip_char: Option<char>) -> Vec<Unit> {
    let mut result: Vec<Unit> = Vec::with_capacity(polymer.len());
    for unit in polymer {
        // Skip this unit if it matches the character we are testing to remove.
        if let Some(ch) = skip_char {
            if ch == unit.kind {
                continue;
            }
        }

        // Check to see if the current unit reacts with the one on top of the stack.
        let reacts = if let Some(other) = result.last() {
            unit.reacts(other)
        } else {
            false
        };

        // If it reacts, remove the top item on the stack, otherwise push this unit.
        if reacts {
            result.pop();
        } else {
            result.push(unit.clone());
        }
    }
    result
}

#[aoc(day5, part1)]
fn solve_part1(polymer: &[Unit]) -> usize {
    react(polymer, None).len()
}

#[aoc(day5, part2)]
fn solve_part2(polymer: &[Unit]) -> usize {
    let reacted = react(polymer, None);
    "abcdefghijklmnopqrstuvwxyz"
        .par_chars()
        .map(|c| react(&reacted, Some(c)).len())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "dabAcCaCBAcCcaDA\n";

    #[test]
    fn test_parse() {
        let expected = &[
            Unit {
                kind: 'd',
                polarity: false,
            },
            Unit {
                kind: 'a',
                polarity: false,
            },
            Unit {
                kind: 'b',
                polarity: false,
            },
            Unit {
                kind: 'a',
                polarity: true,
            },
        ];
        assert_eq!(expected, &parse(INPUT)[..4]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(10, solve_part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, solve_part2(&parse(INPUT)));
    }
}
