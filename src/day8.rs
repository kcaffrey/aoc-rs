#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[u8]) -> u32 {
    fn sum_nodes(n: u16, slice: &[u8]) -> (u32, u16) {
        let mut start = 0;
        let mut sum = 0;
        for _ in 0..n {
            let num_children = slice[start] as u16;
            let num_meta = slice[start + 1];
            let (subtotal, skip) = sum_nodes(num_children, &slice[start + 2..]);
            sum += subtotal;
            start += 2 + usize::from(skip);

            sum += slice[start..start + usize::from(num_meta)]
                .iter()
                .map(|&m| m as u32)
                .sum::<u32>();
            start += usize::from(num_meta);
        }
        (sum, start as u16)
    }
    sum_nodes(1, input).0
}

#[aoc(day8, part2)]
fn solve_part2(input: &[u8]) -> u32 {
    fn get_values(n: u16, slice: &[u8]) -> (Vec<u32>, usize) {
        let mut start = 0;
        let mut values = Vec::new();
        for _ in 0..n {
            let num_children = slice[start] as u16;
            let num_meta = slice[start + 1] as usize;
            let (child_values, skip) = get_values(num_children, &slice[start + 2..]);
            start += 2 + skip;
            if num_children == 0 {
                values.push(
                    slice[start..start + num_meta]
                        .iter()
                        .map(|&m| m as u32)
                        .sum::<u32>(),
                );
            } else {
                values.push(
                    slice[start..start + num_meta]
                        .iter()
                        .map(|&m| child_values.get(m as usize - 1).unwrap_or(&0))
                        .sum(),
                );
            }
            start += num_meta;
        }
        (values, start)
    }
    let (values, _) = get_values(1, input);
    values[0]
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";

    #[test]
    fn test_parse() {
        assert_eq!(
            &[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
            &parse(INPUT)[..]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(138, solve_part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(66, solve_part2(&parse(INPUT)));
    }
}
