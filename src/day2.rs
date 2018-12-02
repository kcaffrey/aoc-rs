use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1<T: AsRef<str>>(input: &[T]) -> i32 {
    let mut two = 0;
    let mut three = 0;
    for s in input {
        let mut counts = HashMap::new();
        for c in s.as_ref().chars() {
            *counts.entry(c).or_insert(0) += 1
        }
        if counts.values().any(|&v| v == 2) {
            two += 1;
        }
        if counts.values().any(|&v| v == 3) {
            three += 1;
        }
    }
    two * three
}

#[aoc(day2, part2)]
pub fn solve_part2<T: AsRef<str>>(input: &[T]) -> String {
    for (index, str1) in input.iter().enumerate() {
        for str2 in input.iter().skip(index + 1) {
            let common = str1
                .as_ref()
                .chars()
                .zip(str2.as_ref().chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect::<String>();
            if common.len() == str1.as_ref().len() - 1 {
                return common;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "abcdef\nbababc\nabbcde";
        assert_eq!(vec!["abcdef", "bababc", "abbcde"], parse(input));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            12,
            solve_part1(&["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",])
        );
    }

    #[test]
    fn test_part2() {
        let input = &[
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!("fgij", solve_part2(input));
    }
}
