use std::collections::HashSet;

// Ran out of time/energy at this point.
// Now (Nov2024) doing some python solutions for the rest
// to get my hand in solving puzzles before Advent of Code 2024

const INPUT: &str = include_str!("inputs/day20.txt");

pub fn day20(input: &str) -> (usize, usize) {

    let mut vals : Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap() ).collect();
    let len = vals.len() as i32;
    let mut inx = 0;
    let mut done : HashSet<i32> = HashSet::new();
    let mut checkuniq : HashSet<i32> = HashSet::new();
    while done.len() < len as usize {
        let val = vals[inx];
        if done.contains(&val) {
            inx += 1;
            continue;
        }
        if checkuniq.contains(&val) {
            panic!("Not uniq");
        }
        checkuniq.insert(val);
        done.insert(val);
        let mut newinx = (val + inx as i32) % (len-1);        
        let old_inx = inx;
        if newinx == 0 {
            newinx = len - 1;
        } else if newinx == len - 1 {
            newinx = 0;
        } else if newinx < 0 { 
            newinx = len + val;           
            inx += 1;
        }

        vals.remove(old_inx);
        vals.insert(newinx as usize, val);        
    }
    println!("{:?}", vals);

    (input.len(), input.len())
}

fn main() {
    let (p1, p2) = day20(INPUT);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("inputs/day20.test1.txt");
        let (p1, p2) = day20(input);

        assert_eq!(p1, input.len());
        assert_eq!(p2, input.len());
    }

    #[test]
    fn test_main() {
        let (p1, p2) = day20(INPUT);

        assert_eq!(p1, INPUT.len());
        assert_eq!(p2, INPUT.len());
    }
}
