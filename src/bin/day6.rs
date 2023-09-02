use std::collections::BTreeSet;
use std::collections::VecDeque;

fn find_marker(input: &str, window_size: usize) -> Option<usize> {
    let mut seen: VecDeque<char> = VecDeque::new();
    for (inx, c) in input.chars().enumerate() {
        seen.push_back(c);
        if seen.len() > window_size {
            seen.pop_front();
        }
        let dupes = seen.iter().collect::<BTreeSet<&char>>();
        if dupes.len() == window_size {
            return Some(inx + 1);
        }
    }
    None
}

pub fn day6(input: &str) -> (usize, usize) {
    (
        find_marker(input, 4).unwrap(),
        find_marker(input, 14).unwrap(),
    )
}

fn main() {
    let input = include_str!("day6.txt");
    let (p1, p2) = day6(input);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(input: &str, expect: (usize, usize)) {
        let res = day6(input);
        println!("{}\n{}", res.0, res.1);
        assert_eq!(res, expect);
    }

    #[test]
    fn test1() {
        run_test("mjqjpqmgbljsphdztnvjfqwrcgsmlb", (7, 19));
    }

    #[test]
    fn test2() {
        run_test("bvwbjplbgvbhsrlpgdmjqwftvncz", (5, 23));
    }

    #[test]
    fn test3() {
        run_test("nppdvjthqldpwncqszvftbrmjlhg", (6, 23));
    }

    #[test]
    fn test4() {
        run_test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", (10, 29));
    }

    #[test]
    fn test5() {
        run_test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", (11, 26));
    }
}
