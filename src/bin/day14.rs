use itertools::Itertools;
use nom::IResult;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("day14.txt");

fn parse_line(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    nom::multi::separated_list0(
        nom::bytes::complete::tag(" -> "),
        nom::sequence::separated_pair(
            nom::character::complete::u32,
            nom::bytes::complete::tag(","),
            nom::character::complete::u32,
        ),
    )(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    nom::multi::separated_list0(nom::character::complete::line_ending, parse_line)(input)
}

pub fn day14(input: &str) -> (usize, usize) {
    let (_, rock_lines) = parse_lines(input).unwrap();

    let mut rock_face: BTreeSet<(u32, u32)> = BTreeSet::new();

    let (mut minx, mut maxx, mut maxy) = (500, 500, 0);
    for line in rock_lines {
        for (from, to) in line.iter().tuple_windows() {
            for x in from.0.min(to.0)..=to.0.max(from.0) {
                for y in from.1.min(to.1)..=to.1.max(from.1) {
                    minx = minx.min(x);
                    maxx = maxx.max(x);
                    maxy = maxy.max(y);
                    rock_face.insert((x, y));
                }
            }
        }
    }

    let mut sands = 0;
    let mut done = false;
    while !done {
        let mut sand = (500, 0);
        let mut falls = true;
        while falls {
            falls = false;
            for s in [sand.0, sand.0 - 1, sand.0 + 1]
            {
                if !rock_face.contains(&(s, sand.1+1)) {
                    sand = (s, sand.1+1);
                    falls = true;
                    break;
                }                
            }
            //println!("{:?}", sand);
            if sand.0 < minx || sand.0 > maxx {
                done = true;
                break;
            }
        }
        if !done {
            sands += 1;
            rock_face.insert(sand);
        }
    }    

    
    (sands, input.len())
}

fn main() {
    println!("1asdasdasdasdd");
    let (p1, p2) = day14(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day14.test1.txt");
        let (p1, p2) = day14(input);

        assert_eq!(p1, 24);
        assert_eq!(p2, 57);
    }

    #[test]    
    fn test_main() {
        let (p1, p2) = day14(INPUT);

        assert_eq!(p1, 913);
        assert_eq!(p2, 23);
    }
}
