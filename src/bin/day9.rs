use std::collections::HashSet;

pub fn day9(input: &str) -> (usize, usize) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert((0,0));
    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);
    for line in input.lines() {
        println!("{}", line);
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
            let delta = (head.0-tail.0, head.1-tail.1);
            if  delta.0.abs() > 1 || delta.1.abs() > 1 {
                tail = (tail.0 + delta.0.signum(), tail.1 + delta.1.signum());
                visited.insert(tail);
            }
            println!("{:?} {:?}", head, tail)
        }
    }
    dbg!(visited.clone());    
    return (visited.len(), input.len());
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
        //assert_eq!(p2, 4);
    }
}
