use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<(char, char)> {
    let re = Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
    input
        .lines()
        .filter_map(|line| {
            let caps = re.captures(line)?;
            Some((
                caps[1].chars().next().unwrap(),
                caps[2].chars().next().unwrap(),
            ))
        })
        .collect()
}

struct DependencyGraph {
    dependencies: HashMap<char, HashSet<char>>,
    dependents: HashMap<char, Vec<char>>,
    available: BinaryHeap<Reverse<char>>,
}

impl<'a, T> From<T> for DependencyGraph
where
    T: IntoIterator<Item = &'a (char, char)>,
{
    fn from(from: T) -> DependencyGraph {
        let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
        let mut dependents: HashMap<char, Vec<char>> = HashMap::new();
        let mut all: HashSet<char> = HashSet::new();
        for (dependency, dependent) in from.into_iter() {
            all.insert(*dependency);
            all.insert(*dependent);
            dependencies
                .entry(*dependent)
                .or_default()
                .insert(*dependency);
            dependents.entry(*dependency).or_default().push(*dependent);
        }
        let available: BinaryHeap<Reverse<char>> = all
            .iter()
            .filter(|step| {
                dependencies
                    .get(*step)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .cloned()
            .map(Reverse)
            .collect();
        DependencyGraph {
            dependencies,
            dependents,
            available,
        }
    }
}

impl DependencyGraph {
    fn complete(&mut self, item: char) {
        if let Some(dependents) = self.dependents.get(&item) {
            for dependent in dependents {
                if let Some(dependencies) = self.dependencies.get_mut(dependent) {
                    dependencies.remove(&item);
                    if dependencies.is_empty() {
                        self.available.push(Reverse(*dependent));
                    }
                }
            }
        }
    }

    fn pop(&mut self) -> Option<char> {
        self.available.pop().map(|c| c.0)
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[(char, char)]) -> String {
    let mut graph = DependencyGraph::from(input);
    let mut ret: Vec<char> = Vec::new();
    while let Some(ch) = graph.pop() {
        ret.push(ch);
        graph.complete(ch);
    }
    ret.iter().collect()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[(char, char)]) -> u32 {
    let mut graph = DependencyGraph::from(input);
    let mut threads: [Option<(u32, char)>; 5] = [None; 5];
    let mut open_threads = 0;
    let mut cur_time = 0;
    while !graph.available.is_empty() || open_threads > 0 {
        for index in 0..threads.len() {
            if let Some((time, ch)) = threads[index] {
                if time <= cur_time {
                    graph.complete(ch);
                    threads[index] = None;
                    open_threads -= 1;
                }
            }
        }
        for index in 0..threads.len() {
            if threads[index].is_none() {
                if let Some(ch) = graph.pop() {
                    threads[index] = Some((cur_time + 60 + u32::from((ch as u8) - b'A' + 1), ch));
                    open_threads += 1;
                }
            }
        }
        cur_time = threads
            .iter()
            .filter_map(|o| if let Some((t, _)) = o { Some(*t) } else { None })
            .min()
            .unwrap_or(cur_time);
    }
    cur_time
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_parse() {
        assert_eq!(&[('C', 'A'), ('C', 'F'), ('A', 'B')], &parse(INPUT)[..3]);
    }

    #[test]
    fn test_part1() {
        assert_eq!("CABDFE", &solve_part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(253, solve_part2(&parse(INPUT)));
    }
}
