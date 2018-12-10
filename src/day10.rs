use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Point> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
    }
    input
        .lines()
        .filter_map(|line| {
            let caps = RE.captures(line)?;
            Some(Point {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                dx: caps[3].parse().unwrap(),
                dy: caps[4].parse().unwrap(),
            })
        })
        .collect()
}

fn find_minimum_origin_distance_time(points: &[Point]) -> i32 {
    // At each time t, a points distance to origin is (x + t * dx)^2 + (y + t * dy)^2
    // We want to find the time t that minimizes the sum of the distances to origin.
    // This works out to t = round(-sum(x * dx + y * dy) / sum(dx^2 + dy^2))
    let numerator: i32 = points.iter().map(|p| p.x * p.dx + p.y * p.dy).sum();
    let denominator: i32 = points.iter().map(|p| p.dx * p.dx + p.dy * p.dy).sum();
    (-(numerator as f32) / (denominator as f32)).round() as i32
}

fn change_time(points: &mut [Point], dt: i32) {
    for point in points {
        point.x += point.dx * dt;
        point.y += point.dy * dt;
    }
}

fn plot_points(points: &[Point]) -> String {
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();
    let mut sky = vec![vec![' '; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for point in points {
        sky[(point.y - miny) as usize][(point.x - minx) as usize] = '#';
    }
    (vec!["\n".to_owned()])
        .into_iter()
        .chain(
            sky.into_iter()
                .map(|line| line.into_iter().collect::<String>()),
        )
        .collect::<Vec<_>>()
        .join("\n")
}

#[aoc(day10, part1)]
fn solve_part1(points: &[Point]) -> String {
    let mut points: Vec<Point> = points.iter().cloned().collect();
    let dt = find_minimum_origin_distance_time(&points);
    change_time(&mut points, dt);
    plot_points(&points)
}

#[aoc(day10, part2)]
fn solve_part2(points: &[Point]) -> i32 {
    find_minimum_origin_distance_time(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_parse() {
        let expected = &[
            Point {
                x: 9,
                y: 1,
                dx: 0,
                dy: 2,
            },
            Point {
                x: 7,
                y: 0,
                dx: -1,
                dy: 0,
            },
        ];
        assert_eq!(expected, &parse(INPUT)[..2]);
    }

    #[test]
    fn test_find_origin_time() {
        assert_eq!(3, find_minimum_origin_distance_time(&parse(INPUT)));
    }

    #[test]
    fn test_change_time() {
        let mut points = parse(INPUT);
        change_time(&mut points, 3);
        let expected = &[
            Point {
                x: 9,
                y: 7,
                dx: 0,
                dy: 2,
            },
            Point {
                x: 4,
                y: 0,
                dx: -1,
                dy: 0,
            },
        ];
        assert_eq!(expected, &points[..2]);
    }
}
