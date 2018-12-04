use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct ShiftLog {
    minute: u32,
    event: Event,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    BeginShift { guard_id: u32 },
    FallAsleep,
    WakeUp,
}

use self::Event::*;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<ShiftLog> {
    let re = Regex::new(
        r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (wakes up|falls asleep|Guard #(\d+) begins shift)$",
    ).unwrap();
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();
    lines
        .iter()
        .filter_map(|line| {
            let caps = re.captures(line)?;
            let minute = caps[1].parse().unwrap();
            let event = if let Some(guard_id) = caps.get(3) {
                BeginShift {
                    guard_id: guard_id.as_str().parse().unwrap(),
                }
            } else {
                match &caps[2] {
                    "wakes up" => WakeUp,
                    "falls asleep" => FallAsleep,
                    _ => unreachable!(),
                }
            };
            Some(ShiftLog { minute, event })
        }).collect()
}

fn solve_with_strat(logs: &[ShiftLog], strat: impl Fn(&HashMap<u32, u32>) -> u32) -> u32 {
    let mut guard_id = 0;
    let mut sleep_time = 0;
    let mut asleep_minutes = HashMap::new();
    for log in logs {
        match log.event {
            BeginShift { guard_id: id } => guard_id = id,
            FallAsleep => sleep_time = log.minute,
            WakeUp => {
                let guard: &mut HashMap<_, _> = asleep_minutes.entry(guard_id).or_default();
                for minute in sleep_time..log.minute {
                    let entry = guard.entry(minute).or_default();
                    *entry += 1;
                }
            }
        }
    }
    let guard = asleep_minutes
        .iter()
        .max_by_key(|(_, minutes)| strat(minutes))
        .map(|(guard_id, _)| guard_id)
        .unwrap();
    let max_minute = asleep_minutes[guard]
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(guard_id, _)| guard_id)
        .unwrap();
    guard * max_minute
}

#[aoc(day4, part1)]
fn solve_part1(logs: &[ShiftLog]) -> u32 {
    solve_with_strat(logs, |minutes| minutes.values().sum::<u32>())
}

#[aoc(day4, part2)]
fn solve_part2(logs: &[ShiftLog]) -> u32 {
    solve_with_strat(logs, |minutes| minutes.values().max().unwrap().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-01 00:55] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_parse() {
        let log = parse(INPUT);
        let expected = &[
            ShiftLog {
                minute: 0,
                event: BeginShift { guard_id: 10 },
            },
            ShiftLog {
                minute: 5,
                event: FallAsleep,
            },
            ShiftLog {
                minute: 25,
                event: WakeUp,
            },
            ShiftLog {
                minute: 30,
                event: FallAsleep,
            },
            ShiftLog {
                minute: 55,
                event: WakeUp,
            },
            ShiftLog {
                minute: 58,
                event: BeginShift { guard_id: 99 },
            },
        ];
        assert_eq!(expected, &log[..=5])
    }

    #[test]
    fn test_part1() {
        assert_eq!(240, solve_part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4455, solve_part2(&parse(INPUT)));
    }
}
