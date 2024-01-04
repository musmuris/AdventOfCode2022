use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day18.txt");

pub fn day18(input: &str) -> (usize, usize) {
    let cubes = input
        .lines()
        .filter_map(|x| {
            x.split(",")
                .map(|i| i.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32, i32)>()
        })
        .collect::<HashSet<_>>();

    let mut droplet = HashSet::<(i32, i32, i32)>::new();
    let mut sides = 0;
    for cube in cubes.iter() {
        sides += 6;
        for d in [(1,0,0), (-1,0,0), (0,1,0), (0,-1,0), (0,0,1), (0,0,-1)]  {
            let lookup = (cube.0 + d.0, cube.1 + d.1, cube.2 + d.2);
            if droplet.contains(&lookup) {
                sides -= 2;
            }
        }
        droplet.insert(*cube);
    }

    let (max_x, max_y, max_z) = cubes.iter().copied()
        .reduce(|(a, b, c), (x, y, z)| (a.max(x+1), b.max(y+1), c.max(z+1)))
        .unwrap();

    //Start from (first cube) and "fill" the outside space
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut ext_sides = 0;
    queue.push_back((-1,-1,-1));
    while let Some(pos) = queue.pop_front() {

        if seen.contains(&pos) { continue; }

        seen.insert(pos.clone());

        for d in [(1,0,0), (-1,0,0), (0,1,0), (0,-1,0), (0,0,1), (0,0,-1)] {
            let lookup = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);
            if cubes.contains(&lookup) {
                ext_sides += 1;
            } else {
                if lookup.0 <= max_x && lookup.1 <= max_y && lookup.2 <= max_z 
                && lookup.0 > -2 && lookup.1 > -2 && lookup.2 > -2 {
                    if !seen.contains(&lookup) {
                        queue.push_back(lookup.clone())
                    }
                }
            }
        }
    }

    (sides, ext_sides)
}

fn main() {
    let (p1, p2) = day18(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day18.test1.txt");
        let (p1, p2) = day18(input);

        assert_eq!(p1, 64);
        assert_eq!(p2, 58);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day18(INPUT);

        assert_eq!(p1, 4504);
        assert_eq!(p2, 2556);
    }
}
