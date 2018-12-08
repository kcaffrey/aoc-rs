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
    fn sum_metas(slice: &[u8]) -> (u32, usize) {
        let num_children = slice[0];
        let num_meta = slice[1] as usize;
        let mut index = 2;
        let mut total = 0;
        for _ in 0..num_children {
            let (subtotal, skip) = sum_metas(&slice[index..]);
            index += skip;
            total += subtotal;
        }
        total += slice[index..index + num_meta]
            .iter()
            .map(|&m| m as u32)
            .sum::<u32>();
        (total, index + num_meta)
    }
    sum_metas(input).0
}

#[aoc(day8, part2)]
fn solve_part2(input: &[u8]) -> u32 {
    fn get_value(slice: &[u8]) -> (u32, usize) {
        let num_children = slice[0];
        let num_meta = slice[1] as usize;
        let mut i = 2;

        // No children, compute sum of metas
        if num_children == 0 {
            let value = slice[i..i + num_meta].iter().map(|&m| m as u32).sum();
            return (value, i + num_meta);
        }

        // Otherwise find child values and use our metas to index into those and get the sum
        let mut child_values = Vec::with_capacity(num_children as usize);
        for _ in 0..num_children {
            let (value, skip) = get_value(&slice[i..]);
            i += skip;
            child_values.push(value);
        }
        let value = slice[i..i + num_meta]
            .iter()
            .map(|&m| child_values.get(m as usize - 1).unwrap_or(&0))
            .sum();
        (value, i + num_meta)
    }
    get_value(input).0
}

struct Node<'a> {
    children: Vec<Node<'a>>,
    meta: &'a [u8],
}

fn build_tree(input: &[u8]) -> (Node, usize) {
    let num_children = input[0];
    let num_meta = input[1] as usize;
    let mut children = Vec::with_capacity(num_children as usize);
    let mut index = 2;
    for _ in 0..num_children {
        let (child, skip) = build_tree(&input[index..]);
        children.push(child);
        index += skip;
    }
    (
        Node {
            children,
            meta: &input[index..index + num_meta],
        },
        index + num_meta,
    )
}

#[aoc(day8, part1, tree)]
fn solve_part1_tree(input: &[u8]) -> u16 {
    fn sum_metas(node: &Node) -> u16 {
        let mut sum = node.children.iter().map(sum_metas).sum();
        sum += node.meta.iter().map(|&m| m as u16).sum::<u16>();
        sum
    }
    sum_metas(&build_tree(input).0)
}

#[aoc(day8, part2, tree)]
fn solve_part2_tree(input: &[u8]) -> u16 {
    fn get_value(node: &Node) -> u16 {
        if node.children.len() == 0 {
            node.meta.iter().map(|&m| m as u16).sum()
        } else {
            let child_values = node.children.iter().map(get_value).collect::<Vec<_>>();
            node.meta
                .iter()
                .map(|&m| child_values.get(m as usize - 1).unwrap_or(&0))
                .sum()
        }
    }
    get_value(&build_tree(input).0)
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
        assert_eq!(138, solve_part1_tree(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(66, solve_part2(&parse(INPUT)));
        assert_eq!(66, solve_part2_tree(&parse(INPUT)));
    }
}
