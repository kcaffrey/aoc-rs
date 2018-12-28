use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Condition {
    immune: Army,
    infection: Army,
    immune_effective_damages: Vec<Vec<u64>>,
    infection_effective_damages: Vec<Vec<u64>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Army {
    groups: Vec<Group>,
    alive_groups: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    units: u64,
    health: u64,
    attack_type: String,
    attack_damage: u64,
    initiative: u64,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ArmyType {
    Immune,
    Infection,
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Box<Condition> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(\d+) units each with (\d+) hit points (\([^)]+\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)")
            .unwrap();
        static ref WEAK: Regex = Regex::new(r"weak to ([a-z]+(, [a-z]+)*)").unwrap();
        static ref IMMUNE: Regex = Regex::new(r"immune to ([a-z]+(, [a-z]+)*)").unwrap();
    }
    let mut armies = input.split("Infection:").map(|input| {
        Army::new(
            RE.captures_iter(input)
                .map(|caps| {
                    let (immunities, weaknesses) = caps
                        .get(3)
                        .map(|s| {
                            (
                                IMMUNE
                                    .captures(s.as_str())
                                    .map(|caps| caps[1].split(", ").map(|s| s.to_owned()).collect())
                                    .unwrap_or_else(Vec::new),
                                WEAK.captures(s.as_str())
                                    .map(|caps| caps[1].split(", ").map(|s| s.to_owned()).collect())
                                    .unwrap_or_else(Vec::new),
                            )
                        })
                        .unwrap_or_else(|| (vec![], vec![]));
                    Group {
                        units: caps[1].parse().unwrap(),
                        health: caps[2].parse().unwrap(),
                        attack_type: caps[5].to_owned(),
                        attack_damage: caps[4].parse().unwrap(),
                        initiative: caps[6].parse().unwrap(),
                        immunities,
                        weaknesses,
                    }
                })
                .collect(),
        )
    });
    Box::new(Condition::new(
        armies.next().unwrap(),
        armies.next().unwrap(),
    ))
}

#[aoc(day24, part1)]
fn solve_part1(condition: &Condition) -> u64 {
    let mut condition = condition.clone();
    while condition.immune.alive_groups > 0 && condition.infection.alive_groups > 0 {
        condition.fight();
    }
    condition.immune.total_units() + condition.infection.total_units()
}

#[aoc(day24, part2)]
fn solve_part2(condition: &Condition) -> u64 {
    // Simulation loop for a single boost amount.
    let fight_with_boost = |boost| {
        let mut immune = condition.immune.clone();
        immune.boost(boost);
        let mut condition = Condition::new(immune, condition.infection.clone());
        while condition.immune.alive_groups > 0 && condition.infection.alive_groups > 0 {
            let units_killed = condition.fight();
            if units_killed == 0 {
                break;
            }
        }
        condition
    };

    // Binary search to find the minimum required boost.
    let (mut boost_min, mut boost_max) = (0, 268_435_456);
    while boost_max > boost_min {
        let boost = (boost_min + boost_max) / 2;
        let condition = fight_with_boost(boost);
        if condition.infection.total_units() == 0 {
            boost_max = boost;
        } else if condition.infection.total_units() > 0 && boost == boost_min {
            boost_min = boost + 1;
        } else {
            boost_min = boost;
        }
    }

    // Return the number of units at that boost level.
    let condition = fight_with_boost(boost_min);
    condition.immune.total_units()
}

impl Condition {
    fn new(immune: Army, infection: Army) -> Condition {
        Condition {
            immune_effective_damages: immune
                .groups
                .iter()
                .map(|a| {
                    infection
                        .groups
                        .iter()
                        .map(|b| b.effective_damage_taken(a))
                        .collect()
                })
                .collect(),
            infection_effective_damages: infection
                .groups
                .iter()
                .map(|a| {
                    immune
                        .groups
                        .iter()
                        .map(|b| b.effective_damage_taken(a))
                        .collect()
                })
                .collect(),
            immune,
            infection,
        }
    }

    fn fight(&mut self) -> u64 {
        // Select targets
        let mut target_selection: BinaryHeap<(u64, ArmyType, usize, usize)> = BinaryHeap::new();
        let mut immune_attacked = HashSet::new();
        let mut infection_attacked = HashSet::new();
        for att_index in self.immune.indexes_by_power() {
            let defender = self.immune_effective_damages[att_index]
                .iter()
                .enumerate()
                .filter(|(i, d)| {
                    **d > 0
                        && self.infection.groups[*i].units > 0
                        && !infection_attacked.contains(i)
                })
                .max_by_key(|(i, d)| {
                    (
                        *d,
                        self.infection.groups[*i].effective_power(),
                        self.infection.groups[*i].initiative,
                    )
                });

            if let Some((def_index, _)) = defender {
                infection_attacked.insert(def_index);
                target_selection.push((
                    self.immune.groups[att_index].initiative,
                    ArmyType::Immune,
                    att_index,
                    def_index,
                ));
            }
        }
        for att_index in self.infection.indexes_by_power() {
            let defender = self.infection_effective_damages[att_index]
                .iter()
                .enumerate()
                .filter(|(i, d)| {
                    **d > 0 && self.immune.groups[*i].units > 0 && !immune_attacked.contains(i)
                })
                .max_by_key(|(i, d)| {
                    (
                        *d,
                        self.immune.groups[*i].effective_power(),
                        self.immune.groups[*i].initiative,
                    )
                });

            if let Some((def_index, _)) = defender {
                immune_attacked.insert(def_index);
                target_selection.push((
                    self.infection.groups[att_index].initiative,
                    ArmyType::Infection,
                    att_index,
                    def_index,
                ));
            }
        }

        // Attack
        let mut units_killed = 0;
        while let Some((_, army, att_index, def_index)) = target_selection.pop() {
            if let (Some(attacker), Some(defender), damage) = match army {
                ArmyType::Immune => (
                    self.immune.groups.get(att_index),
                    self.infection.groups.get_mut(def_index),
                    self.immune_effective_damages[att_index][def_index],
                ),
                ArmyType::Infection => (
                    self.infection.groups.get(att_index),
                    self.immune.groups.get_mut(def_index),
                    self.infection_effective_damages[att_index][def_index],
                ),
            } {
                if attacker.units == 0 || defender.units == 0 {
                    continue;
                }
                units_killed += defender.take_damage(damage * attacker.units);
                if defender.units == 0 {
                    match army {
                        ArmyType::Immune => self.infection.alive_groups -= 1,
                        ArmyType::Infection => self.immune.alive_groups -= 1,
                    }
                }
            }
        }
        units_killed
    }
}

impl Army {
    fn new(groups: Vec<Group>) -> Army {
        Army {
            alive_groups: groups.len(),
            groups,
        }
    }

    fn boost(&mut self, amount: u64) {
        for group in &mut self.groups {
            group.attack_damage += amount;
        }
    }

    fn total_units(&self) -> u64 {
        self.groups.iter().map(|g| g.units).sum()
    }

    fn indexes_by_power(&self) -> impl Iterator<Item = usize> {
        let mut groups = self
            .groups
            .iter()
            .enumerate()
            .filter(|(_, g)| g.units > 0)
            .map(|(i, g)| (g.effective_power(), g.initiative, i))
            .collect::<Vec<_>>();
        groups.sort();
        groups.into_iter().map(|(_, _, i)| i).rev()
    }
}

impl Group {
    fn effective_power(&self) -> u64 {
        self.units * self.attack_damage
    }

    fn effective_damage_taken(&self, other: &Group) -> u64 {
        if self.immunities.iter().any(|s| s == &other.attack_type) {
            return 0;
        }
        if self.weaknesses.iter().any(|s| s == &other.attack_type) {
            return other.attack_damage * 2;
        }
        other.attack_damage
    }

    fn take_damage(&mut self, damage: u64) -> u64 {
        let units_killed = self.units.min(damage / self.health);
        self.units -= units_killed;
        units_killed
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Condition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Immune system:")?;
        self.immune
            .groups
            .iter()
            .enumerate()
            .filter(|(_, g)| g.units > 0)
            .try_for_each(|(i, g)| writeln!(f, "Group {} contains {} units", i + 1, g.units))?;
        writeln!(f, "Infection:")?;
        self.infection
            .groups
            .iter()
            .enumerate()
            .filter(|(_, g)| g.units > 0)
            .try_for_each(|(i, g)| writeln!(f, "Group {} contains {} units", i + 1, g.units))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_parse() {
        let condition = parse(EXAMPLE);
        assert_eq!(condition.immune.groups.len(), 2);
        assert_eq!(condition.infection.groups.len(), 2);
        assert_eq!(
            condition.immune.groups[0],
            Group {
                units: 17,
                health: 5390,
                attack_damage: 4507,
                attack_type: "fire".to_owned(),
                initiative: 2,
                immunities: vec![],
                weaknesses: vec!["radiation".to_owned(), "bludgeoning".to_owned()],
            }
        );
        assert_eq!(
            condition.infection.groups[1].immunities,
            vec!["radiation".to_owned()]
        );
        assert_eq!(
            condition.infection.groups[1].weaknesses,
            vec!["fire".to_owned(), "cold".to_owned()]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(5216, solve_part1(&parse(EXAMPLE)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(51, solve_part2(&parse(EXAMPLE)));
    }
}
