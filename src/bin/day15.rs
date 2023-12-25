use nom::IResult;
use std::cmp::{max, min};

const INPUT: &str = include_str!("day15.txt");

#[derive(Clone)]
#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn can_merge(&self, other: &Self) -> bool {
        self.start <= other.end + 1 && other.start <= self.end + 1
    }

    fn merge(&self, other: &Self) -> Range {
        Range {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
}

struct Position {
    x: i64,
    y: i64,
}
struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    fn range_at_row(&self, row: i64) -> Option<Range> {
        let dist =
            (self.position.x - self.beacon.x).abs() + (self.position.y - self.beacon.y).abs();
        let row_dist = (self.position.y - row).abs();
        let dist_remaining = dist - row_dist;
        if dist_remaining <= 0 {
            None
        } else {
            Some(Range {
                start: self.position.x - dist_remaining,
                end: self.position.x + dist_remaining,
            })
        }
    }

    fn range_at_row_clamped(&self, row: i64, max: i64) -> Option<Range> {
        if let Some(range) = self.range_at_row(row) {
            if range.start > max || range.end < 0 {
                return None;
            } else {
                Some(Range {
                    start: range.start.max(0),
                    end: range.end.min(max),
                })
            }
        } else {
            None
        }
    }
}

fn position_pair(input: &str) -> IResult<&str, Position> {
    let (input, (x, y)) = nom::sequence::separated_pair(
        nom::sequence::preceded(
            nom::bytes::complete::tag("x="),
            nom::character::complete::i64,
        ),
        nom::bytes::complete::tag(", "),
        nom::sequence::preceded(
            nom::bytes::complete::tag("y="),
            nom::character::complete::i64,
        ),
    )(input)?;

    Ok((input, Position { x, y }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Sensor>> {
    let (input, list) = nom::multi::separated_list0(
        nom::character::complete::line_ending,
        nom::sequence::preceded(
            nom::bytes::complete::tag("Sensor at "),
            nom::sequence::separated_pair(
                position_pair,
                nom::bytes::complete::tag(": closest beacon is at "),
                position_pair,
            ),
        ),
    )(input)?;

    Ok((
        input,
        list.into_iter()
            .map(|(position, beacon)| Sensor { position, beacon })
            .collect(),
    ))
}

pub fn day15(input: &str, row: i64, max_dist: i64) -> (i64, i64) {
    let (_, sensors) = parse_lines(input).unwrap();

    let mut result: Vec<Range> = sensors.iter().filter_map(|s| s.range_at_row(row)).collect();
    result.sort_by(|r1, r2| r1.start.cmp(&r2.start));

    let mut merged: Vec<Range> = Vec::new();
    let mut current: Option<Range> = None;
    for i in result {
        current = match current {
            Some(x) => {
                if x.can_merge(&i) {
                    Some(x.merge(&i))
                } else {
                    merged.push(x);
                    None
                }
            }
            None => Some(i),
        };
    }
    if let Some(left) = current {
        merged.push(left);
    }

    let part1 = merged
        .iter()
        .fold(0, |acc, ran| acc + (ran.end - ran.start));

    // Part 2
    let mut found: Option<Position> = None;
    for row in 0..=max_dist {
        let mut result: Vec<Range> = sensors
            .iter()
            .filter_map(|s| s.range_at_row_clamped(row, max_dist))
            .collect();
        result.sort_by(|r1, r2| r1.start.cmp(&r2.start));
        
        let mut merged: Vec<Range> = Vec::new();
        let mut current: Option<Range> = None;
        for i in result {
            current = match current {
                Some(x) => {
                    if x.can_merge(&i) {
                        Some(x.merge(&i))
                    } else {
                        merged.push(x);
                        Some(i)
                    }
                }
                None => Some(i),
            };
        }
        if let Some(left) = current {
            merged.push(left);
        }

        let count = merged
            .iter()
            .fold(0, |acc, ran| acc + (ran.end - ran.start));
        if count < max_dist {
            if found.is_some() {
                panic!("More than one row found")
            };
            if merged.len() > 1 {
                if merged[0].end != merged[1].start - 2 {
                    panic!("Expected single hole")
                }
                found = Some(Position {
                    x: merged[0].end + 1,
                    y: row,
                });
            } else {
                panic!("You need to code the case for start or end of row")
            }
        }
    }
    let f = found.unwrap();
    let part2 =  f.x * 4_000_000 + f.y;

    (part1, part2)
}

fn main() {
    let (p1, p2) = day15(INPUT, 2_000_000, 4_000_000);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day15.test1.txt");
        let (p1, p2) = day15(input, 10, 20);

        assert_eq!(p1, 26);
        assert_eq!(p2, 56000011);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day15(INPUT, 2_000_000, 4_000_000);

        assert_eq!(p1, 4725496);
        assert_eq!(p2, 12051287042458);
    }
}
