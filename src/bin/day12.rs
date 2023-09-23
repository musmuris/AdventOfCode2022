use std::collections::{BTreeMap, VecDeque};

const INPUT: &str = include_str!("day12.txt");

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Pos(usize, usize);

fn grid_walk(grid: &[&[u8]], end: Pos, start_height: u8) -> u32 {
    let mut queue: VecDeque<(Pos, u8)> = VecDeque::new();
    let mut distance: BTreeMap<Pos, u32> = BTreeMap::new();

    distance.insert(end, 0);
    queue.push_back((end, b'z'));

    while !queue.is_empty() {
        let (pos, current_height) = queue.pop_front().unwrap();

        if current_height == start_height {
            return distance[&pos];
        }

        for delta in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
            let neighbour = Pos(
                (pos.0 as isize + delta.0) as usize,
                (pos.1 as isize + delta.1) as usize,
            );
            if !distance.contains_key(&neighbour) {
                if let Some(&height) = grid.get(neighbour.0).and_then(|l| l.get(neighbour.1)) {
                    let real_height = if height == b'S' { b'a' } else { height };
                    if real_height >= current_height - 1 {
                        queue.push_back((neighbour, height));
                        distance.insert(neighbour, distance[&pos] + 1);
                    }
                }
            }
        }
    }
    panic!("No path")
}

pub fn day12(input: &str) -> (u32, u32) {
    let mut end = Pos(0, 0);
    let grid: Vec<&[u8]> = input
        .lines()
        .enumerate()
        .map(|(line_inx, line_str)| {
            let line = line_str.as_bytes();
            if let Some(i) = line.iter().position(|&b| b == b'E') {
                end = Pos(line_inx, i)
            }            
            line
        })
        .collect();

    let p1 = grid_walk(&grid, end, b'S');
    let p2 = grid_walk(&grid, end, b'a');
    
    (p1, p2)
}

fn main() {
    let (p1, p2) = day12(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("day12.test1.txt");
        let (p1, p2) = day12(input);

        assert_eq!(p1, 31);
        assert_eq!(p2, 29);
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day12(INPUT);

        assert_eq!(p1, 520);
        assert_eq!(p2, 508);
    }
}
