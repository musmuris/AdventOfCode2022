use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day3.txt");

pub fn day3(input: &str) -> (u32, u32) {
    let mut acc1: u32 = 0;
    for line in input.lines() {
        if !line.is_ascii() {
            panic!("ASCII only string expected")
        }
        let (first, second) = line.split_at(line.len() / 2);

        let chars1: HashSet<u8> = first.bytes().collect();
        let chars2: HashSet<u8> = second.bytes().collect();
        let sames = chars1.intersection(&chars2);
        let same = sames.at_most_one();

        acc1 += map_priority(*same.unwrap().unwrap())
    }

    let mut acc2 = 0;
    for set in &input.lines().chunks(3) {
        let chunk_lines: Vec<HashSet<u8>> = set.map(|x| x.bytes().collect()).collect();
        let mut found: Option<u8> = None;
        for c in &chunk_lines[0] {
            if chunk_lines[1].contains(c) && chunk_lines[2].contains(c) {
                found = match found {
                    None => Some(*c),
                    Some(_) => panic!("more than one found"),
                }
            }
        }
        if found.is_none() {
            panic!("No match found")
        }

        acc2 += map_priority(found.unwrap())
    }

    (acc1, acc2)
}

fn map_priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("Expected a..z or A..Z characters only"),
    }
    .into()
}

fn main() {
    let (p1, p2) = day3(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day3.test1.txt");

        let (p1, p2) = day3(input);

        assert_eq!(p1, 157);
        assert_eq!(p2, 70);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day3(INPUT);

        assert_eq!(p1, 8252);
        assert_eq!(p2, 2828);
    }
}
