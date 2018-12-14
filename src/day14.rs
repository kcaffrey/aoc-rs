#[aoc_generator(day14)]
fn parse(input: &str) -> String {
    input.to_owned()
}

struct Recipes {
    recipes: Vec<u8>,
    indexes: Vec<usize>,
    target: Option<usize>,
    input_len: usize,
    recent_recipes: usize,
}

impl Recipes {
    fn new(capacity: usize) -> Self {
        let mut recipes: Vec<u8> = Vec::with_capacity(capacity);
        recipes.extend([3, 7].iter());
        let indexes: Vec<usize> = vec![0, 1];
        Recipes {
            recipes,
            indexes,
            target: None,
            input_len: 10000,
            recent_recipes: 37,
        }
    }

    fn cook(&mut self) -> bool {
        let mut sum: u8 = self.indexes.iter().map(|&i| self.recipes[i]).sum();
        if sum >= 10 {
            if self.add_recipe(1) {
                return true;
            }
            sum -= 10;
        }
        if self.add_recipe(sum) {
            return true;
        }
        for i in 0..self.indexes.len() {
            self.indexes[i] = self.indexes[i] + usize::from(self.recipes[self.indexes[i]]) + 1;
            if self.indexes[i] >= self.recipes.len() {
                self.indexes[i] = self.indexes[i] % self.recipes.len();
            }
        }
        false
    }

    fn add_recipe(&mut self, recipe: u8) -> bool {
        self.recipes.push(recipe);
        if let Some(target) = self.target {
            self.recent_recipes = (self.recent_recipes % self.input_len) * 10 + recipe as usize;
            target == self.recent_recipes
        } else {
            false
        }
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
    let input = input_str.trim().parse::<usize>().unwrap();
    let mut recipes = Recipes::new(input);
    recipes.target = Some(input);
    recipes.input_len = 10usize.pow(input_str.len() as u32 - 1);
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
