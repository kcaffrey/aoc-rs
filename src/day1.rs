use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<i32> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    seen.insert(0);
    input
        .iter()
        .cycle()
        .scan(0, |f, c| {
            *f = *f + c;
            Some(*f)
        }).find(|&f| !seen.insert(f))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse() {
        let input = "+1\n-2\n+3\n-4\n";
        assert_eq!(vec![1, -2, 3, -4], parse(input));
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(3, solve_part1(&[1, -2, 3, 1]));
        assert_eq!(3, solve_part1(&[1, 1, 1]));
        assert_eq!(0, solve_part1(&[1, 1, -2]));
        assert_eq!(-6, solve_part1(&[-1, -2, -3]));
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(2, solve_part2(&[1, -2, 3, 1]));
        assert_eq!(0, solve_part2(&[1, -1]));
        assert_eq!(10, solve_part2(&[3, 3, 4, -2, -4]));
        assert_eq!(5, solve_part2(&[-6, 3, 8, 5, -6]));
        assert_eq!(14, solve_part2(&[7, 7, -2, -7, -4]));
    }
}
