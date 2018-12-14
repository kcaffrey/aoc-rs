use std::collections::VecDeque;

#[aoc_generator(day14)]
fn parse(input: &str) -> String {
    input.to_owned()
}

struct Recipes {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
    target: Option<Vec<u8>>,
    recent_recipes: VecDeque<u8>,
}

impl Recipes {
    fn new(capacity: usize) -> Self {
        let mut recipes = Vec::with_capacity(capacity);
        let mut recent = VecDeque::with_capacity(6);
        recipes.extend(&[3, 7]);
        recent.extend(&[3, 7]);
        Recipes {
            recipes,
            elf1: 0,
            elf2: 1,
            target: None,
            recent_recipes: recent,
        }
    }

    fn cook(&mut self) -> bool {
        let mut sum = self.recipes[self.elf1] + self.recipes[self.elf2];
        if sum >= 10 {
            if self.add_recipe(1) {
                return true;
            }
            sum -= 10;
        }
        if self.add_recipe(sum) {
            return true;
        }
        self.elf1 = self.next_index(self.elf1);
        self.elf2 = self.next_index(self.elf2);
        false
    }

    fn add_recipe(&mut self, recipe: u8) -> bool {
        self.recipes.push(recipe);
        if let Some(target) = &self.target {
            if self.recent_recipes.len() >= target.len() {
                self.recent_recipes.pop_front();
            }
            self.recent_recipes.push_back(recipe);
            for i in 0..target.len() {
                if target[i] != self.recent_recipes[i] {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn next_index(&self, cur: usize) -> usize {
        let mut val = cur;
        val = val + usize::from(self.recipes[val]) + 1;
        if val >= self.recipes.len() {
            val = val % self.recipes.len();
        }
        val
    }
}

#[aoc(day14, part1)]
fn solve_part1(input_str: &str) -> String {
    let input = input_str.trim().parse::<usize>().unwrap();
    let mut recipes = Recipes::new(input);
    while recipes.recipes.len() < input + 10 {
        recipes.cook();
    }
    (&recipes.recipes[input..input + 10])
        .iter()
        .map(|r| char::from(b'0' + r))
        .collect::<String>()
        .to_owned()
}

#[aoc(day14, part2)]
fn solve_part2(input_str: &str) -> usize {
    let mut recipes = Recipes::new(22_000_000);
    recipes.target = Some(input_str.chars().map(|ch| (ch as u8) - b'0').collect());
    while !recipes.cook() {}
    recipes.recipes.len() - input_str.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("5158916779", &solve_part1("9"));
        assert_eq!("0124515891", &solve_part1("5"));
        assert_eq!("9251071085", &solve_part1("18"));
        assert_eq!("5941429882", &solve_part1("2018"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, solve_part2("51589"));
        assert_eq!(5, solve_part2("01245"));
        assert_eq!(18, solve_part2("92510"));
        assert_eq!(2018, solve_part2("59414"));
    }
}
