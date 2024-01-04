use std::fmt::Display;

use itertools::Itertools;
use nom::*;

const INPUT: &str = include_str!("inputs/day13.txt");

enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Packet::Num(i) => i.to_string(),
                Packet::List(l) => {
                    format!("[{}]", l.iter().map(|e| e.to_string()).join(", "))
                }
            }
        )
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::Num(l), Packet::List(r)) => vec![Packet::Num(*l)].cmp(r),
            (Packet::List(l), Packet::Num(r)) => l.cmp(&vec![Packet::Num(*r)]),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l == r,
            (Self::List(l), Self::List(r)) => l == r,
            (Self::List(l), Self::Num(r)) => l.eq(&vec![Packet::Num(*r)]),
            (Self::Num(l), Self::List(r)) => vec![Packet::Num(*l)].eq(r),
        }
    }
}

impl Eq for Packet {}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    branch::alt((
        sequence::delimited(
            bytes::complete::tag("["),
            multi::separated_list0(bytes::complete::tag(","), parse_packet),
            bytes::complete::tag("]"),
        )
        .map(Packet::List),
        character::complete::u32.map(Packet::Num),
    ))(input)
}

pub fn day13(input: &str) -> (usize, usize) {
    let (_, data) = multi::separated_list0(
        character::complete::multispace1,
        sequence::pair(
            sequence::terminated(parse_packet, character::complete::line_ending),
            sequence::terminated(parse_packet, character::complete::line_ending),
        ),
    )(input)
    .unwrap();

    let mut count = 0;
    for (inx, pair) in data.iter().enumerate() {
        if pair.0 < pair.1 {
            count += inx + 1
        }
    }

    let mut all = data.iter().fold(Vec::new(), |mut v, d| {
        v.push(&d.0);
        v.push(&d.1);
        v
    });

    let packet2 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let packet6 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
    all.push(&packet2);
    all.push(&packet6);

    all.sort();

    let mut p2 = 1;
    for (i, p) in all.iter().enumerate() {
        if p == &&packet2 || p == &&packet6 {
            p2 *= i + 1;
        }
    }

    (count, p2)
}

fn main() {
    let (p1, p2) = day13(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day13.test1.txt");
        let (p1, p2) = day13(input);

        assert_eq!(p1, 13);
        assert_eq!(p2, 140);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day13(INPUT);

        assert_eq!(p1, 4643);
        assert_eq!(p2, 21614);
    }
}
