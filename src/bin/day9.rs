use std::collections::HashSet;

pub fn day9(input: &str) -> (usize, usize) {
    let mut visited1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited9: HashSet<(i32, i32)> = HashSet::new();

    visited1.insert((0, 0));
    visited9.insert((0, 0));
    let mut head: (i32, i32) = (0, 0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); 9];
    for line in input.lines() {
        let Some((dir, count)) = line.split_once(" ") else { continue; };

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
            for inx in 0..tails.len() {
                let knot = tails[inx];
                let delta = (knot_ahead.0 - knot.0, knot_ahead.1 - knot.1);
                if delta.0.abs() > 1 || delta.1.abs() > 1 {
                    tails[inx] = (knot.0 + delta.0.signum(), knot.1 + delta.1.signum());
                    if inx == 0 {
                        visited1.insert(tails[inx]);
                    }
                    if inx == 8 {
                        visited9.insert(tails[inx]);
                    }
                }
                knot_ahead = tails[inx];
            }
        }
    }
    //dbg!(visited1.clone());
    (visited1.len(), visited9.len())
}

fn main() {
    let input = include_str!("day9.txt");
    let (p1, p2) = day9(input);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (p1, p2) = day9(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
",
        );

        assert_eq!(p1, 13);
        assert_eq!(p2, 1);
    }

    #[test]
    fn test2() {
        let (p1, p2) = day9(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
",
        );

        assert_eq!(p1, 88);
        assert_eq!(p2, 36);
    }
}
