use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point4(i32, i32, i32, i32);

#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<Point4> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)").unwrap();
    }
    input
        .lines()
        .filter_map(|line| {
            let caps = RE.captures(line.trim())?;
            Some(Point4(
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ))
        })
        .collect()
}

#[aoc(day25, part1)]
fn solve_part1(points: &[Point4]) -> u32 {
    let mut adjacency = vec![vec![]; points.len()];
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if points[i].distance(points[j]) <= 3 {
                adjacency[i].push(j);
                adjacency[j].push(i);
            }
        }
    }

    let mut constellations = 0;
    let mut visited = vec![false; points.len()];
    for i in 0..points.len() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        constellations += 1;
        let mut queue = vec![i];
        while let Some(index) = queue.pop() {
            for &j in &adjacency[index] {
                if !visited[j] {
                    queue.push(j);
                    visited[j] = true;
                }
            }
        }
    }
    constellations
}

impl Point4 {
    fn distance(self, other: Point4) -> i32 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let points = parse("-1,2,2,0\n0,0,2,-2\n0,0,0,-2\n-1,2,0,0");
        assert_eq!(
            points,
            vec![
                Point4(-1, 2, 2, 0),
                Point4(0, 0, 2, -2),
                Point4(0, 0, 0, -2),
                Point4(-1, 2, 0, 0)
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            solve_part1(&parse(
                "0,0,0,0\n3,0,0,0\n0,3,0,0\n0,0,3,0\n0,0,0,3\n0,0,0,6\n9,0,0,0\n12,0,0,0"
            ))
        );

        assert_eq!(
            4,
            solve_part1(&parse(
                "-1,2,2,0\n0,0,2,-2\n0,0,0,-2\n-1,2,0,0\n-2,-2,-2,2\n3,0,2,-1\n-1,3,2,2\n-1,0,-1,0\n0,2,1,-2\n3,0,0,0"
            ))
        );

        assert_eq!(
            3,
            solve_part1(&parse(
                "1,-1,0,1\n2,0,-1,0\n3,2,-1,0\n0,0,3,1\n0,0,-1,-1\n2,3,-2,0\n-2,2,0,0\n2,-2,0,-1\n1,-1,0,-1\n3,2,0,2"
            ))
        );

        assert_eq!(
            8,
            solve_part1(&parse(
                "1,-1,-1,-2\n-2,-2,0,1\n0,2,1,3\n-2,3,-2,1\n0,2,3,-2\n-1,-1,1,-2\n0,-2,-1,0\n-2,2,3,-1\n1,2,2,0\n-1,-2,0,-2"
            ))
        );
    }
}
