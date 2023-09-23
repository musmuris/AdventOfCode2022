use std::fmt::Display;

use itertools::Itertools;
use nom::*;

const INPUT: &str = include_str!("day13.txt");

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
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.partial_cmp(r),
            (Packet::Num(l), Packet::List(r)) => vec![Packet::Num(*l)].partial_cmp(r),
            (Packet::List(l), Packet::Num(r)) => l.partial_cmp(&vec![Packet::Num(*r)]),
            (Packet::List(l), Packet::List(r)) => l.partial_cmp(r),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l == r,            
            (Self::List(l), Self::List(r)) => l == r,
            (Self::List(l), Self::Num(r)) => l.eq(&vec![Packet::Num(*r)]),
            (Self::Num(l), Self::List(r)) => vec![Packet::Num(*l)].eq(r)
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    branch::alt((
        sequence::delimited(
            bytes::complete::tag("["),
            multi::separated_list0(bytes::complete::tag(","), parse_packet),
            bytes::complete::tag("]"),
        )
        .map(|v| Packet::List(v)),
        character::complete::u32.map(|n| Packet::Num(n)),
    ))(input)
}

pub fn day13(input: &str) -> (usize, usize) {
    let (input, data) = multi::separated_list0(
        character::complete::multispace1,
        sequence::pair(
            sequence::terminated(parse_packet, character::complete::line_ending),
            sequence::terminated(parse_packet, character::complete::line_ending)
        )
    )(input)
    .unwrap();

    let mut count = 0;
    for (inx, pair) in data.iter().enumerate() {
        if pair.0 < pair.1 { count += inx+1 }
    }

    (count, input.len())
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
        let input = include_str!("day13.test1.txt");
        let (p1, p2) = day13(input);

        assert_eq!(p1, 13);
        assert_eq!(p2, 209);
    }

    #[test]
    #[ignore = "not yet"]
    fn test_main() {
        let (p1, p2) = day13(INPUT);

        assert_eq!(p1, 23);
        assert_eq!(p2, 23);
    }
}
