use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Claim {
    identifier: u32,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Claim {
    fn intersects(&self, other: &Claim) -> bool {
        (other.left >= self.left && other.left <= self.right
            || self.left >= other.left && self.left <= other.right)
            && (other.top >= self.top && other.top <= self.bottom
                || self.top >= other.top && self.top <= other.bottom)
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    input
        .lines()
        .filter_map(|l| {
            let caps = re.captures(l)?;
            let left = caps[2].parse().unwrap();
            let top = caps[3].parse().unwrap();
            let right = left + caps[4].parse::<u32>().unwrap() - 1;
            let bottom = top + caps[5].parse::<u32>().unwrap() - 1;
            Some(Claim {
                identifier: caps[1].parse().unwrap(),
                left,
                top,
                right,
                bottom,
            })
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(claims: &[Claim]) -> i32 {
    let mut covered = HashMap::new();
    let mut covered_twice = 0;
    for claim in claims {
        for x in claim.left..=claim.right {
            for y in claim.top..=claim.bottom {
                let entry = covered.entry((x, y)).or_insert(0);
                if *entry == 1 {
                    covered_twice += 1;
                }
                *entry += 1;
            }
        }
    }
    covered_twice
}

#[aoc(day3, part2)]
pub fn solve_part2(claims: &[Claim]) -> u32 {
    for claim1 in claims.iter() {
        let mut overlaps = false;
        for claim2 in claims.iter() {
            if claim1 != claim2 && claim1.intersects(claim2) {
                overlaps = true;
                break;
            }
        }
        if !overlaps {
            return claim1.identifier;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let claim = Claim {
            identifier: 1,
            left: 2,
            top: 3,
            right: 5,
            bottom: 7,
        };
        assert_eq!(parse("#1 @ 2,3: 4x5"), vec!(claim));
    }

    const INPUT: &str = "
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn test_part1() {
        let claims = parse(INPUT);
        assert_eq!(4, solve_part1(&claims));
    }

    #[test]
    fn test_part2() {
        let claims = parse(INPUT);
        assert_eq!(3, solve_part2(&claims));
    }
}
