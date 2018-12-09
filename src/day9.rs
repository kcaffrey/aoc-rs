use regex::Regex;
use std::collections::VecDeque;

#[aoc_generator(day9)]
fn parse(input: &str) -> Box<(u32, u32)> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    }
    let caps = RE.captures(input.trim()).unwrap();
    Box::new((caps[1].parse().unwrap(), caps[2].parse().unwrap()))
}

fn solve(num_players: u32, last_marble: u32) -> u32 {
    struct Marble {
        score: u32,
        clockwise: u32,
        counter_clockwise: u32,
    }

    let mut circle = Vec::with_capacity(last_marble as usize + 1 - (last_marble as usize / 23));
    circle.push(Marble {
        score: 0,
        clockwise: 0,
        counter_clockwise: 0,
    });
    let mut scores = vec![0; num_players as usize];
    let mut cur = 0;
    let mut player = 0;
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            scores[player as usize] += marble;
            for _ in 0..7 {
                cur = circle[cur as usize].counter_clockwise;
            }
            scores[player as usize] += circle[cur as usize].score;
            let (left, right) = (
                circle[cur as usize].counter_clockwise,
                circle[cur as usize].clockwise,
            );
            circle[left as usize].clockwise = right;
            circle[right as usize].counter_clockwise = left;
            cur = right;
        } else {
            let left = circle[cur as usize].clockwise;
            let right = circle[left as usize].clockwise;
            cur = circle.len() as u32;
            circle.push(Marble {
                score: marble,
                clockwise: right,
                counter_clockwise: left,
            });
            circle[left as usize].clockwise = cur;
            circle[right as usize].counter_clockwise = cur;
        }
        player += 1;
        if player >= num_players {
            player = 0;
        }
    }

    scores.into_iter().max().unwrap()
}

#[aoc(day9, part1)]
fn solve_part1(&(num_players, last_marble): &(u32, u32)) -> u32 {
    solve(num_players, last_marble)
}

#[aoc(day9, part2)]
fn solve_part2(&(num_players, last_marble): &(u32, u32)) -> u32 {
    solve(num_players, last_marble * 100)
}

fn solve_vecdeque(num_players: u32, last_marble: u32) -> u32 {
    let mut circle = VecDeque::with_capacity((last_marble + 1 - (last_marble / 23) * 2) as usize);
    let mut scores = vec![0u32; num_players as usize];
    circle.push_back(0);
    for (marble, player) in (1..=last_marble).zip((0..num_players).cycle()) {
        if marble % 23 == 0 {
            scores[player as usize] += marble;
            for _ in 0..7 {
                let tmp = circle.pop_back().unwrap();
                circle.push_front(tmp);
            }
            scores[player as usize] += circle.pop_back().unwrap();
            let tmp = circle.pop_front().unwrap();
            circle.push_back(tmp);
        } else {
            let tmp = circle.pop_front().unwrap();
            circle.push_back(tmp);
            circle.push_back(marble);
        }
    }
    scores.into_iter().max().unwrap()
}

#[aoc(day9, part1, vecdeque)]
fn solve_part1_vecdeque(&(num_players, last_marble): &(u32, u32)) -> u32 {
    solve_vecdeque(num_players, last_marble)
}

#[aoc(day9, part2, vecdeque)]
fn solve_part2_vecdeque(&(num_players, last_marble): &(u32, u32)) -> u32 {
    solve_vecdeque(num_players, last_marble * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUTS: &[&str] = &[
        "9 players; last marble is worth 25 points",
        "10 players; last marble is worth 1618 points",
        "13 players; last marble is worth 7999 points",
        "17 players; last marble is worth 1104 points\n",
        "21 players; last marble is worth 6111 points",
        "30 players; last marble is worth 5807 points",
    ];

    #[test]
    fn test_parse() {
        assert_eq!(Box::new((10, 1618)), parse(INPUTS[1]));
    }

    #[test]
    fn test_part1() {
        let expected = &[32, 8317, 146373, 2764, 54718, 37305];
        INPUTS
            .iter()
            .zip(expected.iter().cloned())
            .for_each(|(i, e)| assert_eq!(e, solve_part1(&parse(i)), "{}", i));
    }

    #[test]
    fn test_part1_vecdeque() {
        let expected = &[32, 8317, 146373, 2764, 54718, 37305];
        INPUTS
            .iter()
            .zip(expected.iter().cloned())
            .for_each(|(i, e)| assert_eq!(e, solve_part1_vecdeque(&parse(i)), "{}", i));
    }
}
