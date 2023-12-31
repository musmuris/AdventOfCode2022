use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("day18.txt");

pub fn day18(input: &str) -> (usize, usize) {
    let cubes = input.lines().filter_map(|x| {
        x.split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32, i32)>()
    });

    let mut droplet = HashSet::<(i32,i32,i32)>::new();
    let mut sides = 0;
    for cube in cubes {
        sides += 6;
        for d in [(1,0,0), (-1,0,0), (0,1,0), (0,-1,0), (0,0,1), (0,0,-1)] {
            let lookup = (cube.0 + d.0, cube.1 + d.1, cube.2 + d.2);
            if droplet.contains(&lookup) {
                sides -= 2;
            }
        }
        droplet.insert(cube);
    }



    (sides, input.len())
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
        let input = include_str!("day18.test1.txt");
        let (p1, p2) = day18(input);

        assert_eq!(p1, 64);
        assert_eq!(p2, input.len());
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day18(INPUT);

        assert_eq!(p1, 4504);
        assert_eq!(p2, INPUT.len());
    }
}
