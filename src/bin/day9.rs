use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day9.txt");

pub fn day9(input: &str) -> (usize, usize) {
    let mut visited1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited9: HashSet<(i32, i32)> = HashSet::new();

    visited1.insert((0, 0));
    visited9.insert((0, 0));
    let mut head: (i32, i32) = (0, 0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); 9];
    for line in input.lines() {
        let Some((dir, count)) = line.split_once(' ') else { continue; };

        let count = count.parse::<i32>().unwrap();
        let (dx, dy) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0),
        };

        for _ in 0..count {
            head = (head.0 + dx, head.1 + dy);

            let mut knot_ahead = head;
            for (inx, knot) in tails.iter_mut().enumerate() {
                
                let delta = (knot_ahead.0 - knot.0, knot_ahead.1 - knot.1);
                if delta.0.abs() > 1 || delta.1.abs() > 1 {
                    *knot = (knot.0 + delta.0.signum(), knot.1 + delta.1.signum());
                    if inx == 0 {
                        visited1.insert(*knot);
                    }
                    if inx == 8 {
                        visited9.insert(*knot);
                    }
                }
                knot_ahead = *knot;
            }
        }
    }

    (visited1.len(), visited9.len())
}

fn main() {
    let (p1, p2) = day9(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day9.test1.txt");
        let (p1, p2) = day9(input);

        assert_eq!(p1, 13);
        assert_eq!(p2, 1);
    }

    #[test]
    fn test2() {
        let input = include_str!("inputs/day9.test2.txt");
        let (p1, p2) = day9(input);

        assert_eq!(p1, 88);
        assert_eq!(p2, 36);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day9(INPUT);

        assert_eq!(p1, 5683);
        assert_eq!(p2, 2372);
    }
}
